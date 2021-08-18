(define (STDOUT i32) 1)

(defn (print ([string <ptr u8>] [length usize]))
    (write STDOUT string length))

(defn (write ([fd i32] [data <ptr u8>] [size usize]))
    (#asm (mov rax (i32 1))
          (syscall)))

(defn (exit ([exit-code i32]) !)
    (#asm (mov rax (i32 0))
          (syscall)))
