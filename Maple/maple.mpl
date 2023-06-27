int x = 4323
char c = '\n'
float q = x * 2 - 2161 * 2 * 2
int64 o = 1234567890123456789l
fn yeetFn (int x, char c) int { 
	int usdf = 0
	return c + 1
}

fn returnIntArg(int x) int {
	return x
}

fn Cos(float a) float {
    return cos(a)
}

fn pow(float x, int y) float {
    if y > 0 {
        return x * pow(x, y - 1)
    }
    else {
        return 1.0
    }
}
fn factorial(int x) int {
    if x > 0 {
        return x * factorial(x - 1)
    }
    else {
        return 1
    }
}
fn cos2(float x) float {
    float sum = 0
    int i = 0
    float mult = 1
    while i < 10 {
        sum = sum + mult * pow(x, i) / factorial(i)
        mult = mult * -1
        i = i + 2
    }
    return sum
}

// yuhh
x = yeetFn(x, c)
int one = 1
int stupid = -returnIntArg(5) + 2

bool b = false 

if !!b {
	stupid = 10
}
elseif stupid == -4 {
	stupid = 20
}
stupid = 0
int yeet = 11
while yeet > 0 {
	yeet = yeet - 1
	stupid = stupid + 1
}
float PI = 3.141592653589793238462643383279502884197169399375105820974944592307816406286
stupid = 0 - 5 + 2
o = micro()
q = cos(0.0)
o = micro() - o
int64 o2 = micro()
q = cos2(0)
o2 = micro() - o2
stupid = returnIntArg('\n')
