var startNanos = std.time.nanos()
// var sdfsfd = import "maple2.mpl"
var sdfsfd = {}
sdfsfd.x = 1
var a = 1
var b = 0

fn yeet(b) {
    b += 1
}

yeet(b)

b = { 
    a = 0,
    b = fn (a) {
        return a + 1
    }
    1.1 = 2
}
b.x = 1

a += 1

b = b[1.1]

b = [1, 2, 3, 4, 5]

b = b[4 - 1]

b = -4

b = !!true

var time = std.time.nanos() - startNanos


fn printScreen(arr) {
    var i = 0
    var printed = ""
    while i < std.arr.len(arr) {
        var subArr = arr[i]
        var j = 0
        while j < std.arr.len(subArr) {
            printed = printed + subArr[j]
            j += 1
        }
        printed = printed + "\n"
        i += 1
    }
    std.io.print(printed)
}

std.io.println("Time: " + toStr(time / 1000.0) + "us")
std.io.println(sdfsfd.x)

var i = 0 
var spaces = ""
var arr2 = []
while std.arr.len(arr2) < 38 {
    var subArr = []
    while std.arr.len(subArr) < 71 {
        subArr[std.arr.len(subArr)] = " "
    }
    arr2[std.arr.len(arr2)] = subArr
}
var ballPos = {
    x = 0,
    y = 0
}
var direction = {
    x = 1,
    y = 1
}
while i < 100000000 {
    var start = std.time.nanos()
    arr2[ballPos.y][ballPos.x] = " "
        spaces = spaces + " "
    printScreen(arr2)
    arr2[ballPos.y][ballPos.x] = "-"
    ballPos.x = ballPos.x + direction.x
    ballPos.y = ballPos.y + direction.y
    if ballPos.x >= std.arr.len(arr2[0]) {
        direction.x = -1
        ballPos.x = std.arr.len(arr2[0]) - 1
    }
    if ballPos.x < 0 {
        direction.x = 1
        ballPos.x = 0
    }
    if ballPos.y >= std.arr.len(arr2) {
        direction.y = -1
        ballPos.y = std.arr.len(arr2) - 1
    }
    if ballPos.y < 0 {
        direction.y = 1
        ballPos.y = 0
        // break
    }
    i += 1
    while std.time.nanos() - start < 10000000 {
        }
}
