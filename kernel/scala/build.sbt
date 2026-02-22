import scala.scalanative.build.*
import java.nio.file.Paths

enablePlugins(ScalaNativePlugin)

scalaVersion := "3.3.3"

def findClang(name: String): java.nio.file.Path = {
  val fromPath = scala.util.Try(
    sys.process.Process(Seq("which", name)).!!.trim
  ).toOption.filter(_.nonEmpty)
  val candidates = Seq(
    fromPath,
    Some(s"/usr/bin/$name"),
    Some(s"/usr/local/bin/$name"),
    Some(s"/run/current-system/sw/bin/$name"),
  ).flatten
  candidates
    .map(Paths.get(_))
    .find(p => p.toFile.exists && p.toFile.canExecute)
    .getOrElse(Paths.get(name))
}

nativeConfig := {
  val linkerScript = (ThisBuild / baseDirectory).value / ".." / "linker.ld"
  // Empty stub archives for pthread and dl — Scala Native's nativelib links
  // them unconditionally, but they don't exist in a bare-metal -static build.
  // The stubs satisfy the linker; the symbols are never called at runtime.
  val stubsDir = (ThisBuild / baseDirectory).value / "stubs"
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
      s"-Wl,-T," + linkerScript.getCanonicalPath,
      "-Wl,--no-dynamic-linker",
      // Point linker at our empty stub archives so -lpthread and -ldl resolve
      s"-L" + stubsDir.getCanonicalPath,
    ))
    .withCompileOptions(Seq(
      "-fno-stack-protector",
      "-mno-red-zone",
    ))
}
