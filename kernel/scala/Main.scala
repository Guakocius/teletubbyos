import scala.scalanative.unsafe.*
import scala.scalanative.unsigned.*

// VGA text buffer
val VGA_BUFFER: Ptr[UShort] = 0xB8000.asInstanceOf[Ptr[UShort]]
val VGA_WIDTH  = 80
val VGA_HEIGHT = 25

var vgaCol = 0
var vgaRow = 0

def vgaColor(fg: UByte, bg: UByte): UByte =
  (fg | (bg << 4)).toUByte

def vgaEntry(c: UByte, color: UByte): UShort =
  (c.toUShort | (color.toUShort << 8)).toUShort

def vgaClear(): Unit =
  val blank = vgaEntry(' '.toUByte, vgaColor(2.toUByte, 0.toUByte))
  var i = 0
  while i < VGA_WIDTH * VGA_HEIGHT do
    !(VGA_BUFFER + i) = blank
    i += 1
  vgaCol = 0
  vgaRow = 0

def vgaScroll(): Unit =
  var row = 1
  while row < VGA_HEIGHT do
    var col = 0
    while col < VGA_WIDTH do
      !(VGA_BUFFER + (row - 1) * VGA_WIDTH + col) =
        !(VGA_BUFFER + row * VGA_WIDTH + col)
      col += 1
    row += 1
  val blank = vgaEntry(' '.toUByte, vgaColor(2.toUByte, 0.toUByte))
  var col = 0
  while col < VGA_WIDTH do
    !(VGA_BUFFER + (VGA_HEIGHT - 1) * VGA_WIDTH + col) = blank
    col += 1
  vgaRow = VGA_HEIGHT - 1

def vgaPutChar(c: Char, color: UByte): Unit =
  if c == '\n' then
    vgaCol = 0
    vgaRow += 1
    if vgaRow >= VGA_HEIGHT then vgaScroll()
  else
    !(VGA_BUFFER + vgaRow * VGA_WIDTH + vgaCol) =
      vgaEntry(c.toByte.toUByte, color)
    vgaCol += 1
    if vgaCol >= VGA_WIDTH then
      vgaCol = 0
      vgaRow += 1
      if vgaRow >= VGA_HEIGHT then vgaScroll()

def vgaPrint(s: String, color: UByte): Unit =
  s.foreach(c => vgaPutChar(c, color))

// COM1 serial
val COM1: UShort = 0x3F8.toUShort

@extern
def serial_outb(port: UShort, value: UByte): Unit = extern

@extern
def serial_inb(port: UShort): UByte = extern

def serialInit(): Unit =
  serial_outb((COM1.toInt + 1).toUShort, 0x00.toUByte)
  serial_outb((COM1.toInt + 3).toUShort, 0x80.toUByte)
  serial_outb((COM1.toInt + 0).toUShort, 0x03.toUByte)
  serial_outb((COM1.toInt + 1).toUShort, 0x00.toUByte)
  serial_outb((COM1.toInt + 3).toUShort, 0x03.toUByte)
  serial_outb((COM1.toInt + 2).toUShort, 0xC7.toUByte)
  serial_outb((COM1.toInt + 4).toUShort, 0x0B.toUByte)

def serialWriteByte(b: UByte): Unit =
  while (serial_inb((COM1.toInt + 5).toUShort) & 0x20.toUByte) == 0.toUByte do ()
  serial_outb(COM1, b)

def serialPrint(s: String): Unit =
  s.foreach { c =>
    if c == '\n' then serialWriteByte('\r'.toByte.toUByte)
    serialWriteByte(c.toByte.toUByte)
  }

def kernelMain(): Unit =
  vgaClear()
  serialInit()
  serialPrint("TeletubbyOS kernel: booted (Scala Native).\n")
  serialPrint("Status: Teletubbys still contained.\n")
  val green  = vgaColor(2.toUByte,  0.toUByte)
  val cyan   = vgaColor(3.toUByte,  0.toUByte)
  val yellow = vgaColor(14.toUByte, 0.toUByte)
  vgaPrint("  TeletubbyOS\n",                         cyan)
  vgaPrint("  ============\n",                        green)
  vgaPrint("  Kernel booted! (Scala Native)\n",       green)
  vgaPrint("  Status: Teletubbys still contained.\n", yellow)
  while true do ()

// Satisfies sbt's main class requirement — _start is the real entry point
object Main:
  // _start is exported and called directly by Limine — this main is never reached
  @exported("_start")
  def start(): Unit = kernelMain()

  def main(args: Array[String]): Unit = kernelMain()
