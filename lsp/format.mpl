var startNanos = std.time.nanos()
// var sdfsfd = import "maple2.mpl"
var sdfsfd = {

}
sdfsfd.x = 1
var a = 1
var b = 0

const yeet = fn (b) {
    b += 1
}
yeet(b)
b = {
    a = 0,
    b = fn (a) {
        return a + 1
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
const printScreen = fn (arr) {
    var i = 0
    var printed = ""
    while i < std.arr.len(arr) {
        var subArr = arr[i]
        var j = 0
        while j < std.arr.len(subArr) {
            printed = printed + subArr[j]
            j += 1
        }
        printed = printed + "
"
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

const vec = fn (x, y) {
    return {
        x = x,
        y = y
    }
}



var ballPos = vec(0, 0)
var direction = vec(1, 1)
var acc = vec(0, 1)
const isOnScreen = fn (pos) {
    if pos.x < 0 {
        return false
    }  
    if pos.y < 0 {
        return false
    }  
    if pos.y >= w {
        return false
    }  
    if pos.x >= h {
        return false
    }  
    return true
}
var time = std.time.nanos() - startNanos











// std.io.println(sdfsfd.x)






































// std.io.println("Time: " + toStr(time / 1000.0) + "us")
// std.io.print(" ")

// while i < 100000000 {
//     var start = std.time.nanos()
//     // arr2[ballPos.y][ballPos.x] = " "
//     if isOnScreen(ballPos) {
//         arr2[ballPos.y][ballPos.x] = "#"
//         printScreen(arr2)
//     }
//         ballPos.x = ballPos.x + direction.x
//         ballPos.y = ballPos.y + direction.y
//     // direction.x += acc.x
//     // direction.y += acc.y
//     if ballPos.x >= h {
//         direction.x = -direction.x
//         // ballPos.x = w - 1
//     }
//     if ballPos.x < 0 {
//         direction.x = 1
//         // ballPos.x = 0
//     }
//     if ballPos.y >= w {
//         direction.y = std.math.ceil(-direction.y )
//         // ballPos.y = h - 1
//     }
//     if ballPos.y < 0 {
//         direction.y = 1
//         // ballPos.y = 0
//         // break
//     }
//     i += 1
//     var time = std.time.nanos() - start
//     std.time.sleepNanos(100000000 - time)
//     // while std.time.nanos() - start < 10000000 {
//     // }
// }
