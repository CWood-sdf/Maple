fn isOnScreen(pos) {
    if pos.x < 0 {
        return false
    }
    if pos.y < 0{
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
