var time = std.time.nanos()
var x = import maple.mpl

var end = std.time.nanos()
std.io.println("Time: " + toStr((end - time) / 1000000) + "ms")
