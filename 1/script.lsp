(setq input (read-line))

(defun toint (in)
 (map 'list (
    lambda(c)
        (case c
          (#\( 1)
          (#\) -1)
          (otherwise 0))
    ) in)
 )

(write (format NIL "Dest: ~d~%" (reduce '+ (toint input))))
