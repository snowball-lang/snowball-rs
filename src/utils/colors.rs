
#[macro_export]
macro_rules! bold { () => { "\x1b[1m" } }
#[macro_export]
macro_rules! underline { () => { "\x1b[4m" } }
#[macro_export]
macro_rules! red { () => { "\x1b[31m" } }
#[macro_export]
macro_rules! green { () => { "\x1b[32m" } }
#[macro_export]
macro_rules! yellow { () => { "\x1b[33m" } }
#[macro_export]
macro_rules! blue { () => { "\x1b[34m" } }
#[macro_export]
macro_rules! reset { () => { "\x1b[0m" } }
#[macro_export]
macro_rules! black { () => { "\x1b[30m" } }
