#define R_NO_REMAP
#include <R.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "myrustlib/api.h"

// Actual Wrappers
void bootstrap_rs_rs();

// Standard R package stuff
// static const R_CallMethodDef CallEntries[] = {
//   {"bootstrap_rs_rs", (DL_FUNC) &bootstrap_rs_rs, 0},
//   {NULL, NULL, 0}
// };

// void R_init_bootstrapCppRs(DllInfo *dll) {
//   R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
//   R_useDynamicSymbols(dll, FALSE);
// }
