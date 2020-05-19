#define R_NO_REMAP
#define STRICT_R_HEADERS
#include <Rinternals.h>

// Import C headers for rust API
#include "myrustlib/api.h"

// Actual Wrappers
SEXP bootstrap_wrapper(){
  return Rf_ScalarReal(bootstrap());
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
