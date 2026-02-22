import scala.scalanative.unsafe.*
import scala.scalanative.unsigned.*

@extern
object VGA {
  def vga_clear(): Unit = extern
  def vga_set_color(fg: UByte, bg: UByte): Unit = extern
  def vga_putc(ch: UByte): Unit = extern
  def vga_write(ptr: Ptr[Byte], len: CSize): Unit = extern
}

object Console {
  def putc(c: Char): Unit =
    VGA.vga_putc(c.toByte.toUByte)

  def print(s: String): Unit = {
    // primitive, aber sicher: keine fancy libs nötig
    var i = 0
    while (i < s.length) {
      val ch = s.charAt(i)
      if (ch == '\n') putc('\n')
      else putc(ch)
      i += 1
    }
  }

  def println(s: String): Unit = {
    print(s); putc('\n')
  }
}

object Kernel {
  @exported("scala_kernel_main")
  def main(): Unit = {
    VGA.vga_clear()
    VGA.vga_set_color(0x0Fu.toUByte, 0x00u.toUByte)

    Console.println("TeletubbyOS")
    Console.println("------------------------------")
    Console.println("Scala kernel layer: online")
    Console.println("Status: Teletubbys still contained.")

    while (true) {
      // CPU schlafen lassen (wir rufen Rust `hlt` später über extern auf)
    }
  }
}
