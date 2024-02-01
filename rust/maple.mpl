var startNanos = std.time.nanos()
var sdfsfd = import maple2.mpl
// var sdfsfd = {}

// sdfsfd.x = 1


var a = 1

var b = 0

fn yeet (b) {
    b += 1
}

yeet(b)



// b = {
//     a = 0,
//     b = fn (a) {
//         return a + 1
//     },
//     a = fn () {
//         return 1
//     },
//     1.1 = 2
// }
//
//
// b.x = 1
//
// a += 1
// b = b[1.1]
// b = [1, 2, 3, 4, 5]
// b = b[4 - 1]
// b = -4
// b = !!true

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
var w = 46
var start = std.time.nanos()
var x = 0
while x < w {
    var subArr = []
    var y = 0
    while y < h {
        subArr[y] = " "
        y += 1
    }
    arr2[x] = subArr
    x += 1
}

var end = std.time.nanos()
std.io.println(std.str.from((end - start) / 1000000) + "ms")

std.io.println(h * w)
fn vec (x, y) {
    return {
        x = x,
        y = y
    }
}
var ballPos = vec(1, 1)
var direction = vec(1, 1)

var acc = vec(0, 1)
fn isOnScreen (pos) {
    if pos.x < 0 {
        return false
    } elseif pos.y < 0 {
        return false
    } elseif pos.y >= h {
        return false
    } elseif pos.x >= w {
        return false
    } else {
        return true
    }
}
var time = std.time.nanos() - startNanos
// std.io.println(time / 1000000000)
// printScreen(arr2)
// i = 0
// while i < 100000 {
//     ballPos.x += direction.x
//     ballPos.y += direction.y
//     if isOnScreen(ballPos) {
//         arr2[ballPos.x][ballPos.y] = "O"
//         printScreen(arr2)
//     }  
//     if ballPos.x >= w - 1 || ballPos.x <= 1 {
// // ballPos.x = w
//         // ballPos.x = 0
//         direction.x = direction.x * -1
//     }  
//     if ballPos.y >= h - 1 || ballPos.y <= 0 {
//         direction.y = direction.y * -1
// // ballPos.y = h
//     // ballPos.y = 0
//     }  
//     std.time.sleepNanos(50000000)
//     i += 1
// }
// std.io.print("Time: " + toStr(time / 1000000) + "ms\n")
