//! Optional registry dependency catalog for source-based native mods.
//! No runtime behavior is added to an unmodded Horizon game.

#[cfg(all(test, feature = "luau"))]
mod tests {
    #[test]
    fn luau_runtime_executes_changed_sources_without_native_recompilation() {
        let lua = mlua::Lua::new();
        assert_eq!(
            lua.load("local n=0; for i=1,4 do n+=i end; return n")
                .eval::<i64>()
                .unwrap(),
            10
        );
        assert_eq!(lua.load("return 23").eval::<i64>().unwrap(), 23);
    }

    #[test]
    fn lua_vms_keep_separate_globals() {
        let first = mlua::Lua::new();
        let second = mlua::Lua::new();
        first.load("privateValue=42").exec().unwrap();
        assert!(second
            .load("return privateValue")
            .eval::<Option<i64>>()
            .unwrap()
            .is_none());
    }
}
