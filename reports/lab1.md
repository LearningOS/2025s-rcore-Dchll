# 功能
实现了追踪当前任务系统调用的历史信息，在结构体 TaskManagerInner 中添加了 counter 字段，是一个长度为 最大任务数量 x 5 的 isize 数组，通过当前任务 id x 5 + 五个系统调用的偏移量进行对应数据的加1和读取。
1. 如果 trace_request 为 0，则 id 应被视作 *const u8，表示读取当前任务 id 地址处一个字节的无符号整数值。此时应忽略 data 参数。返回值为 id 地址处的值。
1. 如果 trace_request 为 1，则 id 应被视作 *const u8，表示写入 data（作为 u8，即只考虑最低位的一个字节）到该用户程序 id 地址处。返回值应为0。
1. 如果 trace_request 为 2，表示查询当前任务调用编号为 id 的系统调用的次数，返回值为这个调用次数。本次调用也计入统计。
1. 否则，忽略其他参数，返回值为 -1。
# 问答题
1. RustSBI-QEMU Version 0.2.0-alpha.2
```   
    ch2b_bad_address.rs
    PageFault in application, kernel killed it.
    
    ch2b_bad_instructions.rs
    IllegalInstruction in application, kernel killed it.
    
    ch2b_bad_register.rs
    IllegalInstruction in application, kernel killed it.
```
2. 回答
   1. sp 代表内核栈；中断处理后恢复用户态，任务切换或调度后恢复。
   2. 特殊处理了t0~t2寄存器
      ```
      ld t0, 32*8(sp)
      csrw sstatus, t0
      恢复t0寄存器的值，即恢复sstatus寄存器
      恢复进入用户态之前的处理器状态
      
      ld t1, 33*8(sp)
      csrw sepc, t1
      恢复t1寄存器的值，即sepc寄存器
      确保程序能够继续执行中断前的指令
      
      ld t2, 2*8(sp)
      csrw sscratch, t2
      恢复t2寄存器的值，即sscratch寄存器
      恢复用户态程序的sp栈指针，确保可以正确访问用户栈
      ```
   3. x2是sp，它在在之后指向的是内核栈，用户栈的栈指针保存在sscratch中，必须通过csrr指令读到通用寄存器中后才能使用；x4寄存器一般不会被用到
   4. sp指向用户栈，sscrath指向内核栈
   5. 发生在sret指令；执行sret后，会恢复中断或异常之前的执行状态，从sepc指定的地址恢复程序的执行，使处理器切换回用户态执行程序。
   6. sp指向内核栈，sscrath指向用户栈
   7. __alltraps最后的call trap_handler


# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

```
无
```

1. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

```
rCore-Camp-Guide-2025S 文档
rCore-Tutorial-Book-v3 3.6.0-alpha.1 文档
```

1. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

1. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。