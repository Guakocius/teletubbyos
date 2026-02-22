import scala.scalanative.build.*
import java.nio.file.Paths

enablePlugins(ScalaNativePlugin)

scalaVersion := "3.3.3"

// Resolve clang from PATH, falling back to common install locations
def findClang(name: String): java.nio.file.Path = {
  val fromPath = sys.process.Process(Seq("which", name)).lazyLines_!.headOption
  val candidates = Seq(
    fromPath,
    Some(s"/usr/bin/$name"),
    Some(s"/usr/local/bin/$name"),
    Some(s"/run/current-system/sw/bin/$name"), // NixOS
  ).flatten
  candidates
    .map(Paths.get(_))
    .find(p => p.toFile.exists && p.toFile.canExecute)
    .getOrElse(Paths.get(name)) // last resort: let the OS find it
}

nativeConfig := {
  val linkerScript = (ThisBuild / baseDirectory).value / "kernel" / "linker.ld"
  NativeConfig.empty
    .withMode(Mode.releaseFull)
    .withGC(GC.none)
    .withLTO(LTO.full)
    .withOptimize(true)
    .withClang(findClang("clang"))
    .withClangPP(findClang("clang++"))
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