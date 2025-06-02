############################################################################
# BootSection.s
# x86_64 boot section, loading bootloader and switch to protected mode
#
# $Copyright: Copyright (C) village
############################################################################
.org 0
.code16
.section ".text", "ax"

.set estack16,       0x9000
.set estack32,       0x2000000
.set appBaseAddr,    0x100000
.set appBaseSector,  1
.set appSectors,     2879

.global _start
_start:
	movw %cs, %ax
	movw %ax, %ds
	movw %ax, %es
	movw %ax, %ss
	movw $estack16, %bp
	movw %bp, %sp

	call DisplayMsg
	call ReadApplication
	call SwitchToProtectedMode
	jmp  .

# Display boot message
DisplayMsg:
	pusha
    movw $0x0600, %ax       # Clear screen
    movw $0x0700, %bx       # Page 0, white on black
    movw $0x00,   %cx       # left:  (0, 0)
    movw $0x184f, %dx       # right: (80, 50)
    int  $0x10              # Display interrupt

	movw $0x0,    %ax       # Reset es
	movw %ax,     %es
	movw $diskBootMsg, %ax  # Set the display msg address
	movw %ax,     %bp
	movw $0x1301, %ax       # Display string
	movw $0x0007, %bx       # Page 0, Red on black
	movw $26,     %cx       # String length
	movw $0,      %dx       # Show in where, dh: row dl: col
	int  $0x10              # Display interrupt
	popa
	ret

 diskBootMsg: .asciz "Booting from Hard Disk..."

# Loading application from disk 
ReadApplication:
	movw $appSectors,    %cx
	movl $appBaseAddr,   %ebx
	movl $appBaseSector, %esi
_ReadAppData:
	call ReadFromDisk
	addl $1,             %esi
	addl $512,           %ebx
	loop _ReadAppData
	ret

# Read data from disk
ReadFromDisk:
	pushl %ebx
	pusha

	movw $0x1f2, %dx       # 0x1f2
	movb $1,     %al       # read one sector
	out  %al,    %dx

	inc  %dx               # 0x1f3
	movl %esi,   %eax
	out  %al,    %dx

	inc  %dx               # 0x1f4
	movb %ah,    %al
	out  %al,    %dx

	inc  %dx               # 0x1f5
	shrl $16,    %eax
	out  %al,    %dx

	inc  %dx               # 0x1f6
	movb $0xe0,  %al       # LBA28 mode
	orb  %ah,    %al       # LBA address 27 ~ 24
	out  %al,    %dx

	inc  %dx               # 0x1f7
	movb $0x20,  %al       # read cmd
	out  %al,    %dx

_Wait1:
	in   %dx,    %al
	test $0x80,  %al
	jne  _Wait1

_Wait2:
	in   %dx,    %al
	test $0x08,  %al
	je   _Wait2

	movw $256,   %cx
	movw $0x1f0, %dx

_Readw:
	in   %dx,   %ax
	movw %ax,  (%ebx)
	addl $2,    %ebx
	loop _Readw

	popa
	popl %ebx
	ret

# GDT start label
gdtStart:
	# the GDT starts with a null 8-byte
	.long 0x0               # 4 byte
	.long 0x0               # 4 byte

# GDT for code segment. base = 0x00000000, length = 0xfffff for flags
gdtCode: 
	.word 0xffff            # segment length, bits 0-15
	.word 0x0               # segment base, bits 0-15
	.byte 0x0               # segment base, bits 16-23
	.byte 0x9a              # 10011010b # flags (8 bits)
	.byte 0xcf              # 11001111b # flags (4 bits) + segment length, bits 16-19
	.byte 0x0               # segment base, bits 24-31

# GDT for data segment. base and length identical to code segment some flags changed again
gdtData:
	.word 0xffff
	.word 0x0
	.byte 0x0
	.byte 0x92              # 10010010b
	.byte 0xcf              # 11001111b
	.byte 0x0

# GDT end label
gdtEnd:

# GDT descriptor
gdtDescriptor:
	.word gdtEnd - gdtStart - 1 # size (16 bit), always one less of its true size
	.long gdtStart              # address (32 bit)

# define some constants for later use
codeSeg = gdtCode - gdtStart
dataSeg = gdtData - gdtStart

# Switch to protected mode
SwitchToProtectedMode:
	cli                     # disable interrupts
	lgdt gdtDescriptor      # load the GDT descriptor
	mov  %cr0, %eax 
	or   $0x1, %eax         # set 32-bit mode bit in cr0
	mov  %eax, %cr0
	ljmp $codeSeg, $Setup   # far jump by using a different segment

# Setup segment, stack and goto bootloader
.code32
Setup:
	movw $dataSeg, %ax      # update segment
	movw %ax, %ds
	movw %ax, %ss
	movw %ax, %es
	movw %ax, %fs
	movw %ax, %gs
	
	movl $estack32, %ebp    # update stack
	movl %ebp, %esp
	
	jmp *(appBaseAddr)      # jmp to application
	jmp  .

# boot section end
bootSectionEnd:
	.org	510
	.word	0xaa55          # Magic word
