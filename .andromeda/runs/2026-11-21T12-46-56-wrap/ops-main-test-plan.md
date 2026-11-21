@row_retire n=1 row=decision/key-after-dropped-anonymous-block seen=1 detector=none
why: A focused control whose anonymous block was dropped no longer keeps focus: the post-layout fixup gives focus up, and the key acts only after a re-focus, which the code and key_press_after_container_lost_its_box hold.
basis: fct-4mnmyqn6, fct-mfew6peh
because: The post-layout focus fixup makes the row's «focus stays and the key still acts» false; the heir the proposer offered is not master content (fit not_master type 4), so the row retires with no heir.
