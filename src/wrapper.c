#define R_NO_REMAP
#include <R.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "myrustlib/api.h"

// Actual Wrappers
SEXP bootstrap_wrapper(SEXP x, int R){
  int n = length(x);
  return Rf_ScalarReal(Rf_allocVector3(bootstrap(), n));
}

// Standard R package stuff
static const R_CallMethodDef CallEntries[] = {
  {"bootstrap_wrapper", (DL_FUNC) &bootstrap_wrapper, 0},
  {NULL, NULL, 0}
};

void R_init_hellorust(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
