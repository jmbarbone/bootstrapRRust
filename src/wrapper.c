#define R_NO_REMAP
#define STRICT_R_HEADERS
#include <R.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "myrustlib/api.h"

// Actual Wrappers
SEXP bootstrap_wrapper(SEXP x, SEXP R){
  int n = Rf_asInteger(R);
  SEXP out = PROTECT(Rf_allocVector(REALSXP, n));
  double* result = bootstrap_rs(Rf_asReal(x), n);
  
  for(int i = 0; i < n; i++) {
    REAL(out)[i] = result[i];
  }
  
  UNPROTECT(1);
  
  return out;
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
