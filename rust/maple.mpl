var startNanos = std.time.nanos()
var sdfsfd = import maple2.mpl
var sdfsfd = {}

sdfsfd.x = 1


var a = 1

var b = 0

fn yeet (b) {
    b += 1
}

yeet(b)

b = {
    a = 0,
    b = fn (a) {
        return a + 1
    },
    a = fn () {
        return 1
    },
    1.1 = 2
}


b.x = 1

a += 1
b = b[1.1]
b = [1, 2, 3, 4, 5]
b = b[4 - 1]
b = -4
b = !!true
fn printScreen (arr) {
    var i = 0
    var printed = ""
    while i < std.arr.len(arr) {
        // sdf
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
var i = 0
var spaces = ""
var arr2 = []
var h = 71
var w = 35
while std.arr.len(arr2) < w {
    var subArr = []
    while std.arr.len(subArr) < h {
        subArr[std.arr.len(subArr)] = " "
    }
    arr2[std.arr.len(arr2)] = subArr
}
fn vec (x, y) {
    return {
        x = x,
        y = y
    }
}
var ballPos = vec(0, 0)
var direction = vec(1, 1)

var acc = vec(0, 1)
fn isOnScreen (pos) {
    if pos.x < 0 {
        return false
    } elseif pos.y < 0 {
        return false
    } elseif pos.y >= w {
        return false
    } elseif pos.x >= h {
        return false
    } else {
        return true
    }
}
var time = std.time.nanos() - startNanos
// std.io.print("Time: " + toStr(time / 1000000) + "ms\n")
