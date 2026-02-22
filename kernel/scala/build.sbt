import scala.scalanative.build.*
import java.nio.file.Paths

enablePlugins(ScalaNativePlugin)

scalaVersion := "3.3.3"

nativeConfig := {
  val linkerScript = (ThisBuild / baseDirectory).value / "kernel" / "linker.ld"
  NativeConfig.empty
    .withMode(Mode.releaseFull)
    .withGC(GC.none)
    .withLTO(LTO.full)
    .withOptimize(true)
    .withClang(Paths.get("/run/current-system/sw/bin/clang"))
    .withClangPP(Paths.get("/run/current-system/sw/bin/clang++"))
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
