/* Send: a type T is Send if it is safe to move a T across a thread boundary
Sync: a type T is Sync if it is safe to move a &T across a thread boundary. */

/* T is send if it's safe to move T to another thread
destructors will run in the thread T is moved to
=> Can we allocate T in one thread and dealloc it in another? */

/* T is Sync if it's safe to access a T from multiple threads at the same time */
/* T is Sync if and only if &T is Send */

/* Send + Sync:
- i8, f32, bool, char, &str
- slices
- String, Vec, Box
- Arc,
- Mutex
- Atomic* - AmoticBool etc. */

/* Send + !Sync - usually interior mut
- mpsc::Sender & Receiver
- Cell
- RefCell
*/

/* !Send + Sync
MutexGuard - uses OS level primitives, which must be dealloc'd on same thread that created them */

/* !Send + !Sync
- Rc,
- *const T, *mut T*/

/*TODO: exercises: https://google.github.io/comprehensive-rust/exercises/concurrency/morning.html*/
