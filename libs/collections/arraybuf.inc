(struct (ArrayBuf T)
    (length usize)
    (capacity usize)
    (memory (Option (NonNullPtr T))))

{impl (ArrayBuf T)
    (define (new (ArrayBuf T))
        (ArrayBuf (length 0) (capacity 0) (None)))

    (define (push self (value T))
        (cond ((< length capacity)
               (set memory[length] value)
               (set length (1+ length)))
              ((none? memory)
               (allocate (* 2 (size-of T)))
               (set capacity 2)
               (self.push value))
              (else
               (realloc (unwrap memory) (* 2 capacity (size-of T)))
               (set capacity (* 2 capacity))
               (self.push value))))
}
