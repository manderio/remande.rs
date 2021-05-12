import java.time.LocalDateTime

data class Remander(val title: String, val contents: String, val time: LocalDateTime)

data class ToDo(val title: String, val contents: String)

data class ListWithTitle<T>(val title: String, val items: List<T>) {
    infix operator fun plus(item: T): ListWithTitle<T> {
        return this.copy(items = this.items + item)
    }

    infix operator fun minus(item: T): ListWithTitle<T> {
        return this.copy(items = this.items - item)
    }
}

fun main() {
    println("hello world uwu")
}