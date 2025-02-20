///
/// 写一个 RecentCounter 类来计算特定时间范围内最近的请求。
/// 请你实现 RecentCounter 类：
/// RecentCounter() 初始化计数器，请求数为 0 。
/// int ping(int t) 在时间 t 添加一个新请求，其中 t 表示以毫秒为单位的某个时间，并返回过去 3000 毫秒内发生的所有请求数（包括新请求）。确切地说，返回在 [t-3000, t] 内发生的请求数。
/// 保证 每次对 ping 的调用都使用比之前更大的 t 值。

#[test]
fn test_leedcode_933() {
    use crate::queue::simple_queue::Queue;

    struct RecentCounter {
        record: Queue<i32>,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl RecentCounter {
        fn new() -> Self {
            RecentCounter {
                record: Queue::new(10000),
            }
        }

        fn ping(&mut self, t: i32) -> i32 {
            let _ = self.record.enqueue(t);
            while self.record.peek().unwrap().clone() < t - 3000 {
                self.record.dequeue();
            }
            self.record.size() as i32
        }
    }

    let mut rc = RecentCounter::new();
    let ans = rc.ping(1);
    assert_eq!(1, ans);
    let ans = rc.ping(100);
    assert_eq!(2, ans);
    let ans = rc.ping(3001);
    assert_eq!(3, ans);
    let ans = rc.ping(3002);
    assert_eq!(3, ans);
}
