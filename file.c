#include <stdio.h>
#include <math.h>

void print_number(double n){
	if (n==(long long)n) {
		printf("%lld\n", (long long)n);
	} else {
		printf("%lf\n", (double)n);
	}
}
double VOaYUDHxHSSVDve9_g(double x){
	return (x*182);
}

double KgS8fzGOwnvWRHxD_f(double x,double y){
	double total=0;
	for (int i=(int)(VOaYUDHxHSSVDve9_g(x)+5);(VOaYUDHxHSSVDve9_g(x)+5)>(VOaYUDHxHSSVDve9_g(y)*5)?(int)i>(int)(VOaYUDHxHSSVDve9_g(y)*5):(int)i<(int)(VOaYUDHxHSSVDve9_g(y)*5);(VOaYUDHxHSSVDve9_g(x)+5)>(VOaYUDHxHSSVDve9_g(y)*5)?i--:i++){
	total=(total+i);
	}
	return total;
}

int main(){
	print_number(KgS8fzGOwnvWRHxD_f(1,10));
	return 0;
}
