#|
(define (print (string <ptr u8>) (length usize))
    (write string length 1))
|#

(defn (print ([string <ptr u8>] [length usize]))
    (write string length 1))

;(defn (i32->string ([i i32]) !)
    ;(if (= i 0) "0")
    ;(let out (String::new))
    ;(while (> i 0)
    ;    (out.push (% i 10))
    ;    (set i (/ i 10)))
;    out)

(defn (x () i32)
    5)

#|
(define (handle_application ([t Token] [tokens Tokens] [input str]) <Result Ast>)
    (let application (Vec::new))
    (application.push (Ast::Identifier t))
    (while (let (Some token) (tokens.next))
        (match token
            ((t
|#

#|
(define (write (data <ptr u8>) (size usize) (port i32))
    (#asm (mov rsi (usize data))
          (mov rdx (usize size))
          (mov rax (i32 port))
          (mov rdi (i32 1))
          (syscall)))
|#

(defn (write ([data <ptr u8>] [size usize] [port i32]))
    (#asm (mov rsi (usize data))
          (mov rdx (usize size))
          (mov rax (i32 port))
          (mov rdi (i32 1))
          (syscall)))

#|
(define (exit (exit-code i32) !)
    (#asm (mov rax (i32 exit-code))
          (mov rdi (i32 0))
          (syscall)))
|#

(defn (exit ([exit-code i32]) !)
    (#asm (mov rax (i32 exit-code))
          (mov rdi (i32 0))
          (syscall)))
