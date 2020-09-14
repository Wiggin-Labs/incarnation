(defn (print ([string <ptr u8>] [length usize]))
    (write string length 1))

(defn (x () i32)
    5)

(defn (write ([data <ptr u8>] [size usize] [port i32]))
    (#asm (mov rsi (usize data))
          (mov rdx (usize size))
          (mov rax (i32 port))
          (mov rdi (i32 1))
          (syscall)))

(defn (exit ([exit-code i32]) !)
    (#asm (mov rax (i32 exit-code))
          (mov rdi (i32 0))
          (syscall)))
