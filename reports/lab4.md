# 编程题

## 代码迁移：
    直接迁移即可，只需稍微修改一下spawn，因为文件系统的引入，我们需要从文件系统中加载用户程序
``` Rust
pub fn sys_spawn(_path: *const u8) -> isize {
    //...
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let data = app_inode.read_all();
        let new_task = current_task.spawn(data.as_slice());
        // ...
    }
    // ...
}
```

## 实现linkat

    仿照create写了一个linkat
    根据old_name去读取old_inode_id，然后检测new_name是否存在，写入一个新的new_name，其inode_id为old_inode_id

## 实现unlinkat

    根据面向testcase编程 (笑
    unlinkat就直接暴力查找，如果找到对应的dirent，将其改为DirEntry::empty()

## 实现fstat

    fstat主要是在获取ino, mode, nlink三个部分
    也是根据面向testcase编程，nlink直接暴力查找即可
    主要还是ino的获取，Inode将自己的block_id和block_offset传给fs，然后我们自己通过这两个参数算出ino
    因为把ino算错了，导致nlink暴力搜索了一个错误的ino，导致返回的nlink一直是错的

# 简答题

## 1. 在我们的easy-fs中，root inode起着什么作用？如果root inode中的内容损坏了，会发生什么？

    ROOT_INODE是根目录所对应的inode，如果ROOT_INODE损坏，整个文件系统也无法正确运行

## 2. 举出使用 pipe 的一个实际应用的例子。

    将一个命令的输出作为另一个命令的输入

## 3. 如果需要在多个进程间互相通信，则需要为每一对进程建立一个管道，非常繁琐，请设计一个更易用的多进程通信机制。

    共享内存？

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    《你交流的对象说明》
    无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    《你参考的资料说明》
    无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。