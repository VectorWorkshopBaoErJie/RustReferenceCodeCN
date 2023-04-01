
//! 应用于这个crate的隐式匿名模块的文档注释

fn main() {
    println!("这 Rust 注释语法的演示！");
}

pub mod outer_module {

    //!  - 内部行文档
    //!! - 仍是内部行文档 (但开始有感叹号)

    /*!  - 内部块文档 */
    /*!! - 仍是内部块文档 (但开始有感叹号) */

    //   - 仅是注释
    ///  - 外部行文档 (正好3斜线)
    //// - 仅是注释

    /*   - 仅是注释 */
    /**  - 外部块文档 (正好2星号) */
    /*** - 仅是注释 */
    
    pub mod inner_module {}

    pub mod nested_comments {
        /* 在 Rust /* 可以 /* 嵌套注释 */ */ */

        // 所有三种类型的块注释都可以包含或嵌套任何其他类型的注释:

        /*   /* */  /** */  /*! */  */
        /*!  /* */  /** */  /*! */  */
        /**  /* */  /** */  /*! */  */
        pub mod dummy_item {}
    }

    pub mod degenerate_cases {
        // 空内部行文档
        //!

        // 空内部块文档
        /*!*/

        // 空行注释
        //

        // 空外部行文档
        ///

        // 空块注释
        /**/

        pub mod dummy_item {}

        // 空的2星号块不是块文档，而是块注释。
        /***/

    }
    /* The next one isn't allowed because outer doc comments
       require an item that will receive the doc */

}
