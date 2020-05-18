
#include <Rcpp.h>
using namespace Rcpp;

//' Bootstrap C++
//' 
//' @param x A vector of numbers
//' @param R Number of bootstraps
//' @export
// [[Rcpp::export]]
NumericVector bootstrap_cpp(NumericVector x, int R) {
  // Pull the estimate and the size
  double estimate = mean(x);
  int n = x.size();
  
  NumericVector res = NumericVector(R);
  
  for( int i=0; i < R; i++ ) {
    NumericVector sample = Rcpp::sample(x, n, TRUE);
    
    // Run stats on random data
    double sample_estimate = mean(sample);
    double se = sd(sample) / sqrt(n);
    if(se == 0)
      se = 1 / (2 * n);
    
    double out = (sample_estimate - estimate) / se;
    
    if(out == R_PosInf || out == R_NegInf)
      out = R_NaN;
    
    res[i] = out;
  }
  
  return res;
}
