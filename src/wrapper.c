#define R_NO_REMAP
#include <R.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "myrustlib/api.h"

// Actual Wrappers
SEXP bootstrap_wrapper(SEXP x, SEXP R){
  void bootstrap_rs(x, R);
}

// Standard R package stuff
static const R_CallMethodDef CallEntries[] = {
  {"bootstrap_wrapper", (DL_FUNC) &bootstrap_wrapper, 2},
  {NULL, NULL, 0}
};

void R_init_bootstrapRRust(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
