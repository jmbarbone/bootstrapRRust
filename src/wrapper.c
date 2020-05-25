#define R_NO_REMAP
#define STRICT_R_HEADERS
#include <R.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "myrustlib/api.h"

// Actual Wrappers
SEXP bootstrap_wrapper(SEXP x, SEXP R){
  int resamples = Rf_asInteger(R);
  int n = Rf_length(x);
  SEXP out = PROTECT(Rf_allocVector(REALSXP, resamples));
  // vector, resamples, size of vector
  double *result = bootstrap_rs(REAL(x), resamples, n);
  
  // output will be the resamples
  for(int i = 0; i < resamples; i++) {
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
