# 编程题

## 在TaskControlBlock中添加了syscall-times，用于记录task对于每个系统调用的次数。
```Rust 
    /// The task control block (TCB) of a task.
    #[derive(Copy, Clone)]
    pub struct TaskControlBlock {
        /// The task status in it's lifecycle
        pub task_status: TaskStatus,
        /// The task context
        pub task_cx: TaskContext,
        /// The numbers of syscall called by task
        pub syscall_times: [u32; MAX_SYSCALL_NUM],
    }
```
## 在syscall中添加一行，每次syscall时触发计数。
```Rust
    pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
        ...
        cnt_syscall(syscall_id);    
        ...
    }

```

## 实现sys_task_info。
```Rust
    pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
        trace!("kernel: sys_task_info");
        unsafe {
            *_ti = TaskInfo {
                status: TaskStatus::Running,
                syscall_times: get_syscall_times(),
                time: get_time_ms(),
            }; 
        }
        0
    }

```

# 简答题
## 1.  Q: 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

        A: 略

## 2.  Q: 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:

### 1.  Q: L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。
        A: a0是argument0，表示传入的第一个参数，也就是栈顶指针。__restore是恢复之前保存的寄存器和状态，可以用于系统调用返回，中断处理返回等。

### 2.  Q: L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
        A: 特殊处理了sstatus, sepc, sscratch这三个寄存器。
           sstatus: 一个状态寄存器，包含了处理器的当前状态信息,恢复sstatus确保返回用户态时具有正确状态
           sepc: 保存了处理器需要返回的指令地址，恢复sepc使得处理器能够在处理完异常或中断后继续执行被中断的程序
           sscratch: 临时寄存器，此处保存了用户栈指针，临时存放，使得处理器能够在处理完异常或中断后继续执行被中断的程序
    
### 3.  Q: L50-L56：为何跳过了 x2 和 x4? 
        A: x2应该是2*8(sp)，但是这里是栈指针，此时已经存在sscratch中。x4是tp线程指针寄存器，一般用不到（?

### 4.  Q: L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
        A: sscratch是内核栈指针， sp是用户栈指针
    
### 5.  Q: __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
        A: L61: sret吧 sstatus寄存器总保存了当前的特权级别，并且sepc中已经保存了返回地址，sp也指向了用户栈。sret执行后，程序才进入了用户态
    
### 6.  Q: L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？
        A: sscratch是用户栈指针， sp是内核栈指针

### 7.  Q: 从 U 态进入 S 态是哪一条指令发生的？
        A: IDK

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    《你交流的对象说明》
    无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    《你参考的资料说明》
    无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。