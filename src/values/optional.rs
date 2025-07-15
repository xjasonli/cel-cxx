use std::ops::{Deref, DerefMut};

/// CEL-compatible optional value type.
///
/// `Optional<T>` is similar to Rust's `Option<T>` but designed specifically for CEL's
/// optional value semantics. It provides a comprehensive API for working with optional
/// values in CEL expressions.
///
/// # Type Parameters
///
/// - `T`: The type of the optional value
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{Optional, Value};
///
/// // Create optional values
/// let some_val = Optional::new("hello");
/// let none_val: Optional<String> = Optional::none();
///
/// // Convert from/to Option
/// let option = Some(42i64);
/// let optional = Optional::from_option(option);
/// let back_to_option = optional.into_option();
///
/// // Chain operations
/// let result = Optional::new(10)
///     .map(|x| x * 2)
///     .filter(|&x| x > 15)
///     .or(Optional::new(0));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Optional<T>(Box<Option<T>>);

impl<T> Optional<T> {
    /// Creates a new optional value containing the given value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new(42);
    /// assert!(opt.as_option().is_some());
    /// ```
    pub fn new(value: T) -> Self {
        Self(Box::new(Some(value)))
    }

    /// Creates an empty optional value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt: Optional<i32> = Optional::none();
    /// assert!(opt.as_option().is_none());
    /// ```
    pub fn none() -> Self {
        Self(Box::new(None))
    }

    /// Converts the optional into a standard `Option`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new(42);
    /// let option = opt.into_option();
    /// assert_eq!(option, Some(42));
    /// ```
    pub fn into_option(self) -> Option<T> {
        *self.0
    }

    /// Creates an optional from a standard `Option`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let option = Some(42);
    /// let opt = Optional::from_option(option);
    /// assert_eq!(opt.into_option(), Some(42));
    /// ```
    pub fn from_option(opt: Option<T>) -> Self {
        Self(Box::new(opt))
    }

    /// Returns a reference to the contained `Option`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new(42);
    /// assert_eq!(opt.as_option(), &Some(42));
    /// ```
    pub fn as_option(&self) -> &Option<T> {
        &self.0
    }

    /// Returns a mutable reference to the contained `Option`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let mut opt = Optional::new(42);
    /// *opt.as_option_mut() = Some(100);
    /// assert_eq!(opt.into_option(), Some(100));
    /// ```
    pub fn as_option_mut(&mut self) -> &mut Option<T> {
        &mut self.0
    }

    /// Converts from `Optional<T>` to `Optional<&T>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new("hello".to_string());
    /// let opt_ref = opt.as_ref();
    /// assert_eq!(opt_ref.into_option(), Some(&"hello".to_string()));
    /// ```
    pub fn as_ref(&self) -> Optional<&T> {
        Optional::from_option(self.as_option().as_ref())
    }

    /// Converts from `Optional<T>` to `Optional<&mut T>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let mut opt = Optional::new("hello".to_string());
    /// let opt_mut = opt.as_mut();
    /// // Can modify the contained value through opt_mut
    /// ```
    pub fn as_mut(&mut self) -> Optional<&mut T> {
        Optional::from_option(self.as_option_mut().as_mut())
    }

    /// Maps an `Optional<T>` to `Optional<U>` by applying a function to the contained value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new(5);
    /// let doubled = opt.map(|x| x * 2);
    /// assert_eq!(doubled.into_option(), Some(10));
    /// ```
    pub fn map<U, F>(self, f: F) -> Optional<U>
    where
        F: FnOnce(T) -> U,
    {
        Optional::from_option(self.into_option().map(f))
    }

    /// Calls the provided closure with the contained value (if any).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new(42);
    /// let result = opt.inspect(|x| println!("Value: {}", x));
    /// assert_eq!(result.into_option(), Some(42));
    /// ```
    pub fn inspect<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        Optional::from_option(self.into_option().inspect(f))
    }

    /// Returns the optional if it contains a value, otherwise returns an empty optional.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new(5);
    /// let filtered = opt.filter(|&x| x > 3);
    /// assert_eq!(filtered.into_option(), Some(5));
    ///
    /// let opt = Optional::new(2);
    /// let filtered = opt.filter(|&x| x > 3);
    /// assert_eq!(filtered.into_option(), None);
    /// ```
    pub fn filter<P>(self, predicate: P) -> Self
    where
        P: FnOnce(&T) -> bool,
    {
        Optional::from_option(self.0.filter(predicate))
    }

    /// Returns the optional if it contains a value, otherwise returns `other`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let some = Optional::new(2);
    /// let none: Optional<i32> = Optional::none();
    /// let other = Optional::new(100);
    ///
    /// assert_eq!(some.or(other.clone()).into_option(), Some(2));
    /// assert_eq!(none.or(other).into_option(), Some(100));
    /// ```
    pub fn or(self, other: Optional<T>) -> Optional<T> {
        Optional::from_option(self.0.or(other.into_option()))
    }

    /// Returns the optional if it contains a value, otherwise calls `f` and returns the result.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let none: Optional<i32> = Optional::none();
    /// let result = none.or_else(|| Optional::new(42));
    /// assert_eq!(result.into_option(), Some(42));
    /// ```
    pub fn or_else<F>(self, f: F) -> Optional<T>
    where
        F: FnOnce() -> Optional<T>,
    {
        Optional::from_option(self.into_option().or_else(|| f().into_option()))
    }

    /// Returns `Some` if exactly one of `self`, `other` is `Some`, otherwise returns `None`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let some = Optional::new(2);
    /// let none: Optional<i32> = Optional::none();
    ///
    /// assert_eq!(some.clone().xor(none.clone()).into_option(), Some(2));
    /// assert_eq!(none.xor(some).into_option(), Some(2));
    /// ```
    pub fn xor(self, other: Optional<T>) -> Optional<T> {
        Optional::from_option(self.0.xor(other.into_option()))
    }

    /// Returns the optional if it contains a value, otherwise returns an empty optional of type `U`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let some = Optional::new(2);
    /// let other = Optional::new("hello");
    /// let result = some.and(other);
    /// assert_eq!(result.into_option(), Some("hello"));
    /// ```
    pub fn and<U>(self, other: Optional<U>) -> Optional<U> {
        Optional::from_option(self.0.and(other.into_option()))
    }

    /// Returns an empty optional if the optional is empty, otherwise calls `f` with the wrapped value and returns the result.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new(5);
    /// let result = opt.and_then(|x| {
    ///     if x > 0 {
    ///         Optional::new(x * 2)
    ///     } else {
    ///         Optional::none()
    ///     }
    /// });
    /// assert_eq!(result.into_option(), Some(10));
    /// ```
    pub fn and_then<U, F>(self, f: F) -> Optional<U>
    where
        F: FnOnce(T) -> Optional<U>,
    {
        Optional::from_option(self.into_option().and_then(|t| f(t).into_option()))
    }

    /// Takes the value out of the optional, leaving an empty optional in its place.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let mut opt = Optional::new(42);
    /// let taken = opt.take();
    /// assert_eq!(taken.into_option(), Some(42));
    /// assert_eq!(opt.into_option(), None);
    /// ```
    pub fn take(&mut self) -> Optional<T> {
        Optional::from_option(self.0.take())
    }

    /// Takes the value out of the optional if the predicate returns `true`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let mut opt = Optional::new(42);
    /// let taken = opt.take_if(|x| *x > 40);
    /// assert_eq!(taken.into_option(), Some(42));
    /// ```
    pub fn take_if<P>(&mut self, predicate: P) -> Optional<T>
    where
        P: FnOnce(&mut T) -> bool,
    {
        Optional::from_option(self.0.take_if(predicate))
    }

    /// Replaces the actual value in the optional with the value given in parameter, returning the old value if present.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let mut opt = Optional::new(42);
    /// let old = opt.replace(100);
    /// assert_eq!(old.into_option(), Some(42));
    /// assert_eq!(opt.into_option(), Some(100));
    /// ```
    pub fn replace(&mut self, value: T) -> Optional<T> {
        Self::from_option(self.0.replace(value))
    }

    /// Zips `self` with another optional.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt1 = Optional::new(1);
    /// let opt2 = Optional::new("hello");
    /// let zipped = opt1.zip(opt2);
    /// assert_eq!(zipped.into_option(), Some((1, "hello")));
    /// ```
    pub fn zip<U>(self, other: Optional<U>) -> Optional<(T, U)> {
        Optional::from_option(self.into_option().zip(other.into_option()))
    }

    /// Converts from `Optional<T>` (or `&Optional<T>`) to `Optional<&T::Target>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new("hello".to_string());
    /// let deref_opt = opt.as_deref();
    /// assert_eq!(deref_opt.into_option(), Some("hello"));
    /// ```
    pub fn as_deref(&self) -> Optional<&<T as Deref>::Target>
    where
        T: Deref,
    {
        Optional::from_option(self.0.as_deref())
    }

    /// Converts from `Optional<T>` (or `&mut Optional<T>`) to `Optional<&mut T::Target>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let mut opt = Optional::new("hello".to_string());
    /// let deref_mut_opt = opt.as_deref_mut();
    /// // Can modify the dereferenced value
    /// ```
    pub fn as_deref_mut(&mut self) -> Optional<&mut <T as Deref>::Target>
    where
        T: DerefMut,
    {
        Optional::from_option(self.0.as_deref_mut())
    }
}

impl<T, U> Optional<(T, U)> {
    /// Unzips an optional containing a tuple into a tuple of optionals.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt = Optional::new((1, "hello"));
    /// let (opt1, opt2) = opt.unzip();
    /// assert_eq!(opt1.into_option(), Some(1));
    /// assert_eq!(opt2.into_option(), Some("hello"));
    /// ```
    pub fn unzip(self) -> (Optional<T>, Optional<U>) {
        let (v, w) = self.into_option().unzip();
        (Optional::from_option(v), Optional::from_option(w))
    }
}

impl<T> Optional<&T> {
    /// Maps an `Optional<&T>` to an `Optional<T>` by copying the contents of the optional.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt_ref = Optional::new(&42);
    /// let opt_owned = opt_ref.copied();
    /// assert_eq!(opt_owned.into_option(), Some(42));
    /// ```
    pub fn copied(self) -> Optional<T>
    where
        T: Copy,
    {
        Optional::from_option(self.0.copied())
    }

    /// Maps an `Optional<&T>` to an `Optional<T>` by cloning the contents of the optional.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let hello_string = "hello".to_string();
    /// let opt_ref = Optional::new(&hello_string);
    /// let opt_owned = opt_ref.cloned();
    /// assert_eq!(opt_owned.into_option(), Some("hello".to_string()));
    /// ```
    pub fn cloned(self) -> Optional<T>
    where
        T: Clone,
    {
        Optional::from_option(self.0.cloned())
    }
}

impl<T> Optional<&mut T> {
    /// Maps an `Optional<&mut T>` to an `Optional<T>` by copying the contents of the optional.
    pub fn copied(self) -> Optional<T>
    where
        T: Copy,
    {
        Optional::from_option(self.0.copied())
    }

    /// Maps an `Optional<&mut T>` to an `Optional<T>` by cloning the contents of the optional.
    pub fn cloned(self) -> Optional<T>
    where
        T: Clone,
    {
        Optional::from_option(self.0.cloned())
    }
}

impl<T, E> Optional<Result<T, E>> {
    /// Transposes an `Optional` of a `Result` into a `Result` of an `Optional`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let opt_result: Optional<Result<i32, &str>> = Optional::new(Ok(42));
    /// let result_opt = opt_result.transpose();
    /// assert_eq!(result_opt, Ok(Optional::new(42)));
    /// ```
    pub fn transpose(self) -> Result<Optional<T>, E> {
        self.into_option()
            .transpose()
            .map(|option| Optional::from_option(option))
    }
}

impl<T> Optional<Optional<T>> {
    /// Flattens an `Optional<Optional<T>>` into an `Optional<T>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Optional;
    ///
    /// let nested = Optional::new(Optional::new(42));
    /// let flattened = nested.flatten();
    /// assert_eq!(flattened.into_option(), Some(42));
    /// ```
    pub fn flatten(self) -> Optional<T> {
        match self.into_option() {
            Some(inner) => inner,
            None => Optional::none(),
        }
    }
}

impl<T> Deref for Optional<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Optional<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Implements conversion from Option<T>
impl<T> From<Option<T>> for Optional<T> {
    fn from(opt: Option<T>) -> Self {
        Self::from_option(opt)
    }
}

// Implements conversion to Option<T>
impl<T> From<Optional<T>> for Option<T> {
    fn from(opt: Optional<T>) -> Self {
        opt.into_option()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Optional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_option() {
            Some(v) => write!(f, "optional({v})"),
            None => write!(f, "optional.none()"),
        }
    }
}
