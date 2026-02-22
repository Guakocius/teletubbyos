import scala.scalanative.build.*

enablePlugins(ScalaNativePlugin)

scalaVersion := "3.3.3"

// Tell nativeLink specifically that there is no main class
Compile / nativeLink / mainClass := None

nativeConfig := {
  val linkerScript = (ThisBuild / baseDirectory).value / "kernel" / "linker.ld"
  NativeConfig.empty
    .withMode(Mode.releaseFull)
    .withGC(GC.none)
    .withLTO(LTO.full)
    .withOptimize(true)
    .withLinkingOptions(Seq(
      "-nostdlib",
      "-static",
      s"-Wl,-T${linkerScript.absolutePath}",
      "-Wl,--no-dynamic-linker",
    ))
    .withCompileOptions(Seq(
      "-ffreestanding",
      "-fno-stack-protector",
      "-mno-red-zone",
    ))
}
