.OnAttach <- function(libname, pkgname) {
  packageStartupMessage(cat(
    "Welcome to my test package.\n",
    "The only thing that matters is that you run this function:\n",
    "\trun_benchmarks()"))
}
