#[doc(hidden)]
#[macro_export]
macro_rules! __wundergraph_expand_optional_argument {
    ($name: expr,
     $arg_ty: ty,
     $registry: ident,
     $entity: ident,
     $info: expr, true $(, $rest: expr)*) => {
        let arg = $registry.arg_with_default::<Option<$arg_ty>>($name, &None, &$info);
        $entity = $entity.argument(arg);
        __wundergraph_expand_optional_argument!($name, $arg_ty, $registry, $entity, $info $(, $rest )*)
    };
    ($name: expr,
     $arg_ty: ty,
     $registry: ident,
     $entity: ident,
     $info: expr, false $(, $rest: expr)*) => {
        __wundergraph_expand_optional_argument!($name, $arg_ty, $registry, $entity, $info $(, $rest )*)
    };
    ($name:expr, $arg_ty: ty, $registry: ident, $entity: ident, $info: expr) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __wundergraph_expand_limit {
    ($registry: ident, $entity: ident, $info: ident, ) => {
        __wundergraph_expand_optional_argument!("limit", i32, $registry, $entity, $info, true)
    };
    ($registry: ident, $entity: ident, $info: ident, $(,$limit: tt)+) => {
        __wundergraph_expand_optional_argument!("limit", i32, $registry, $entity, $info $(,$limit)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __wundergraph_expand_offset {
    ($registry: ident, $entity: ident, $info: ident, ) => {
        __wundergraph_expand_optional_argument!("offset", i32, $registry, $entity, $info, true)
    };
    ($registry: ident, $entity: ident, $info: ident, $(,$offset: tt)+) => {
        __wundergraph_expand_optional_argument!("offset", i32, $registry, $entity, $info $(,$offset)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __wundergraph_expand_order {
    ($registry: ident, $entity: ident, $conn: ty, $graphql_struct: ident ) => {
        __wundergraph_expand_optional_argument!("order",
                                                Vec<$crate::order::OrderBy<$graphql_struct, <$conn as $crate::diesel::Connection>::Backend>>,
                                                $registry, $entity, &Default::default(), true)
    };
    ($registry: ident, $entity: ident, $conn: ty, $graphql_struct: ident, $(,$order: tt)+) => {
        __wundergraph_expand_optional_argument!("order",
                                                Vec<$crate::order::OrderBy<$graphql_struct, <$conn as $crate::diesel::Connection>::Backend>>,
                                                $registry, $entity, &Default::default() $(,$order)*)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "postgres")]
macro_rules! __wundergraph_expand_pg_loading_handler {
    (
        $query_name: ident {
            $($entity_name: ident(
                $graphql_struct: ident
                $(, filter = $filter_name: ident)*
                $(, limit = $limit: tt)*
                $(, offset = $offset: tt)*
                $(, order = $order: tt)*
            ),)*
        }
    ) => {
        __wundergraph_expand_graphql_type_for_query!{
            $crate::diesel::PgConnection,
            $query_name(context = $crate::diesel::r2d2::PooledConnection<$crate::diesel::r2d2::ConnectionManager<$crate::diesel::PgConnection>>) {
                $($entity_name(
                    $graphql_struct
                    $(, filter = $filter_name)*
                    $(, limit = $limit)*
                    $(, offset = $offset)*
                    $(, order = $order)*
                ),)*
            }
        }
    };
    (
        $query_name: ident(context = $($context:tt)::+<Conn>) {
            $($entity_name: ident(
                $graphql_struct: ident
                $(, filter = $filter_name: ident)*
                $(, limit = $limit: tt)*
                $(, offset = $offset: tt)*
                $(, order = $order: tt)*
            ),)*
        }
    ) => {
        __wundergraph_expand_graphql_type_for_query!{
            $crate::diesel::PgConnection,
            $query_name(context = $($context)::+<$crate::diesel::PgConnection>) {
                $($entity_name(
                    $graphql_struct
                    $(, filter = $filter_name)*
                    $(, limit = $limit)*
                    $(, offset = $offset)*
                    $(, order = $order)*
                ),)*
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "sqlite")]
macro_rules! __wundergraph_expand_sqlite_loading_handler {
    (
        $query_name: ident {
            $($entity_name: ident(
                $graphql_struct: ident
                $(, filter = $filter_name: ident)*
                $(, limit = $limit: tt)*
                $(, offset = $offset: tt)*
                $(, order = $order: tt)*
            ),)*
        }
    ) => {
        __wundergraph_expand_graphql_type_for_query!{
            $crate::diesel::SqliteConnection,
            $query_name(context = $crate::diesel::r2d2::PooledConnection<$crate::diesel::r2d2::ConnectionManager<$crate::diesel::SqliteConnection>>) {
                $($entity_name(
                    $graphql_struct
                    $(, filter = $filter_name)*
                    $(, limit = $limit)*
                    $(, offset = $offset)*
                    $(, order = $order)*
                ),)*
            }
        }
    };
    (
        $query_name: ident(context = $($context:tt)::+<Conn>) {
            $($entity_name: ident(
                $graphql_struct: ident
                $(, filter = $filter_name: ident)*
                $(, limit = $limit: tt)*
                $(, offset = $offset: tt)*
                $(, order = $order: tt)*
            ),)*
        }
    ) => {
        __wundergraph_expand_graphql_type_for_query!{
            $crate::diesel::SqliteConnection,
            $query_name(context = $($context)::+<$crate::diesel::SqliteConnection>) {
                $($entity_name(
                    $graphql_struct
                    $(, filter = $filter_name)*
                    $(, limit = $limit)*
                    $(, offset = $offset)*
                    $(, order = $order)*
                ),)*
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "postgres"))]
// https://github.com/rust-lang-nursery/rustfmt/issues/2749
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! __wundergraph_expand_pg_loading_handler {
    (
        $query_name:ident $((context = $($context:tt)*))*
        {
            $(
                $entity_name:ident(
                    $graphql_struct:ident
                    $(,filter = $filter_name:ident)*
                    $(,limit = $limit:tt)*
                    $(,offset = $offset:tt)*
                    $(,order = $order:tt)*
                ),
            )*
         }
    ) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "sqlite"))]
// https://github.com/rust-lang-nursery/rustfmt/issues/2749
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! __wundergraph_expand_sqlite_loading_handler {
    (
        $query_name:ident $((context = $($context:tt)*))*
        {
            $(
                $entity_name:ident(
                    $graphql_struct:ident
                    $(,filter = $filter_name:ident)*
                    $(,limit = $limit:tt)*
                    $(,offset = $offset:tt)*
                    $(,order = $order:tt)*
                ),
            )*
         }
    ) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __wundergraph_expand_graphql_type_for_query {
    ($conn:ty,
     $query_name: ident(context = $context: ty) {
         $($entity_name: ident(
             $graphql_struct: ident
                 $(, filter = $filter_name: ident)*
                 $(, limit = $limit: tt)*
                 $(, offset = $offset: tt)*
                 $(, order = $order: tt)*
         ),)*
     }
    )=> {
        impl $crate::juniper::GraphQLType<$crate::scalar::WundergraphScalarValue>
            for $query_name<$crate::diesel::r2d2::Pool<$crate::diesel::r2d2::ConnectionManager<$conn>>>
        {
            type Context = $context;
            type TypeInfo = ();

            fn name(_info: &Self::TypeInfo) -> Option<&str> {
                Some(stringify!($query_name))
            }

            #[allow(non_snake_case)]
            fn meta<'r>(
                info: &Self::TypeInfo,
                registry: &mut $crate::juniper::Registry<'r, $crate::scalar::WundergraphScalarValue>
            ) -> $crate::juniper::meta::MetaType<'r, $crate::scalar::WundergraphScalarValue>
                where $crate::scalar::WundergraphScalarValue: 'r
            {
                let fields = &[
                    $(
                        {
                            let mut field = registry.field::<Vec<$crate::graphql_type::GraphqlWrapper<$graphql_struct, <$conn as $crate::diesel::Connection>::Backend>>>(
                                concat!(stringify!($graphql_struct), "s"),
                                info
                            );

//                            $(
                                let filter = registry.arg_with_default::<Option<
                                    $crate::filter::Filter<<$graphql_struct as $crate::LoadingHandler<<$conn as $crate::diesel::Connection>::Backend>>::Filter,    <$graphql_struct as $crate::diesel::associations::HasTable>::Table>>>
                                   //      $crate::filter::filter_helper::FilterWrapper<
                                // <$crate::filter::filter_helper::FilterConverter<
                                //     <<$graphql_struct as $crate::LoadingHandler<_>>::FieldList as $crate::query_helper::placeholder::FieldListExtratcor>::Out,
                                //     <$graphql_struct as $crate::LoadingHandler<_>>::Columns,
                                //     $graphql_struct,
                                //     <$conn as $crate::diesel::Connection>::Backend
                                // > as $crate::filter::filter_helper::CreateFilter>::Filter,
                                //              $graphql_struct,
                                //             <$conn as $crate::diesel::Connection>::Backend
                                //         >,
                                //         <$graphql_struct as $crate::diesel::associations::HasTable>::Table,>>>
                                    ("filter", &None, &$crate::helper::NameBuilder::default());
                                field = field.argument(filter);
  //                          )*
                                __wundergraph_expand_limit!(registry, field, info, $(, $limit)*);
                               __wundergraph_expand_offset!(registry, field, info, $(, $offset)*);
                               __wundergraph_expand_order!(registry, field, $conn, $graphql_struct $(, $order)*);
                            field
                        },
                        {
                            let key_info = $crate::helper::primary_keys::PrimaryKeyInfo::new(&<$graphql_struct as $crate::diesel::associations::HasTable>::table());
                            let key = registry.arg::<
                                $crate::helper::primary_keys::PrimaryKeyArgument<
                                'static,
                            <$graphql_struct as $crate::diesel::associations::HasTable>::Table,
                            $context,
                            <&'static $graphql_struct as $crate::diesel::Identifiable>::Id
                                >
                                >("primaryKey", &key_info);
                            registry.field::<Option<$crate::graphql_type::GraphqlWrapper<$graphql_struct, <$conn as $crate::diesel::Connection>::Backend>>>(
                                stringify!($graphql_struct),
                                info
                            ).argument(key)
                        }
                       ,

                    )*
                ];
                registry.build_object_type::<Self>(info, fields).into_meta()
            }

            fn resolve_field(
                &self,
                _info: &Self::TypeInfo,
                field_name: &str,
                _arguments: &$crate::juniper::Arguments<$crate::scalar::WundergraphScalarValue>,
                executor: &$crate::juniper::Executor<Self::Context, $crate::scalar::WundergraphScalarValue>,
            ) -> $crate::juniper::ExecutionResult<$crate::scalar::WundergraphScalarValue> {
                use $crate::LoadingHandler;
                use $crate::WundergraphContext;
                match field_name {
                    $(
                        concat!(stringify!($graphql_struct), "s") => {
                            let ctx = executor.context();
                            let conn = ctx.get_connection();
                            let look_ahead = executor.look_ahead();
                            let q = $graphql_struct::build_query(&look_ahead)?;
                            let items = $graphql_struct::load(&look_ahead, conn, q)?;
                            Ok($crate::juniper::Value::List(items))
                        },
                        stringify!($graphql_struct) => {
                            let ctx = executor.context();
                            let conn = ctx.get_connection();
                            let look_ahead = executor.look_ahead();
                            let q = $graphql_struct::build_query(&look_ahead)?;
                            let item = $graphql_struct::load_by_primary_key(&look_ahead, conn, q)?;
                            Ok(item.unwrap_or($crate::juniper::Value::Null))
                        }
                    )*
                        e => Err($crate::juniper::FieldError::new(
                            "Unknown field:",
                            $crate::juniper::Value::scalar(e),
                        )),
                }
            }

            fn concrete_type_name(&self, _context: &Self::Context, _info: &Self::TypeInfo) -> String {
                String::from(stringify!($query_name))
            }
        }

        impl $query_name<$crate::diesel::r2d2::Pool<$crate::diesel::r2d2::ConnectionManager<$conn>>>
        {
            // fn handle_filter<T, Ctx>(
  //               &self,
  //               e: &$crate::juniper::Executor<Ctx, $crate::scalar::WundergraphScalarValue>,
  //               s: $crate::juniper::LookAheadSelection<$crate::scalar::WundergraphScalarValue>,
  //           ) -> $crate::juniper::ExecutionResult<$crate::scalar::WundergraphScalarValue>
  //           where
  //               T: $crate::LoadingHandler<<$conn as $crate::diesel::Connection>::Backend>,
  //               <<T as $crate::diesel::associations::HasTable>::Table as $crate::diesel::QuerySource>::FromClause: $crate::diesel::query_builder::QueryFragment<<$conn as $crate::diesel::Connection>::Backend>,
  //              <T as $crate::diesel::associations::HasTable>::Table: 'static,
  //              <T as $crate::diesel::associations::HasTable>::Table:
  //               $crate::diesel::query_dsl::methods::BoxedDsl<'static,
  //                   <$conn as $crate::diesel::Connection>::Backend,
  //                   Output = BoxedSelectStatement<
  //                       'static,
  //                       $crate::diesel::dsl::SqlTypeOf<<<T as $crate::diesel::associations::HasTable>::Table as $crate::diesel::Table>::AllColumns>,
  //                       <T as $crate::diesel::associations::HasTable>::Table,
  //                       <$conn as $crate::diesel::Connection>::Backend
  //                   >
  //               >,
  //             <<T as $crate::LoadingHandler<<$conn as $crate::diesel::Connection>::Backend>>::Filter as $crate::filter::build_filter::BuildFilter<<$conn as $crate::diesel::Connection>::Backend>>::Ret: $crate::diesel::AppearsOnTable<<T as $crate::diesel::associations::HasTable>::Table>
  //           {

  //              let ctx = e.context();

  //              let q = T::build_query(s)?;
  // //              let items = T::load(s, ctx.get_connection(), q)?;

  //               //            Ok($crate::juniper::Value::List(items))
  //               unimplemented!()
  //           }
        }

        //     fn handle_by_key<T, Ctx>(
        //         &self,
        //         e: &$crate::juniper::Executor<Ctx, $crate::scalar::WundergraphScalarValue>,
        //         s: $crate::juniper::LookAheadSelection<$crate::scalar::WundergraphScalarValue>,
        //     ) -> $crate::juniper::ExecutionResult<$crate::scalar::WundergraphScalarValue>
        //     where
        //         T: $crate::LoadingHandler<<$conn as $crate::diesel::Connection>::Backend, Context = Ctx> + 'static
        //         + $crate::juniper::GraphQLType<$crate::scalar::WundergraphScalarValue, TypeInfo = ()>,
        //         T::Table: $crate::diesel::associations::HasTable<Table = T::Table>,
        //         Ctx: $crate::WundergraphContext<<$conn as $crate::diesel::Connection>::Backend>,
        //     <T as $crate::juniper::GraphQLType<$crate::scalar::WundergraphScalarValue>>::Context: $crate::juniper::FromContext<Ctx>,
        //     &'static T: $crate::diesel::Identifiable,
        //     <&'static T as $crate::diesel::Identifiable>::Id: $crate::helper::primary_keys::UnRef<'static>,
        //         $crate::helper::primary_keys::PrimaryKeyArgument<
        //         'static,
        //         T::Table,
        //     Ctx,
        //     <&'static T as $crate::diesel::Identifiable>::Id,
        //     >: $crate::helper::FromLookAheadValue,
        //     <T::Table as $crate::diesel::Table>::PrimaryKey: $crate::diesel::EqAll<<<&'static T as $crate::diesel::Identifiable>::Id as $crate::helper::primary_keys::UnRef<'static>>::UnRefed>,
        //     <<T::Table as $crate::diesel::Table>::PrimaryKey as $crate::diesel::EqAll<<<&'static T as $crate::diesel::Identifiable>::Id as $crate::helper::primary_keys::UnRef<'static>>::UnRefed>>::Output: $crate::diesel::AppearsOnTable<T::Table> + $crate::diesel::expression::NonAggregate + $crate::diesel::query_builder::QueryFragment<<$conn as $crate::diesel::Connection>::Backend>,
        //     {
        //         use $crate::diesel::QueryDsl;

        //         let ctx = e.context();
        //         let q = T::default_query().into_boxed();
        //         let item = T::load_item(&s, ctx, q)?;
        //         e.resolve_with_ctx(&(), &item)
        //     }
        // }
    }
}

#[macro_export]
macro_rules! wundergraph_query_object {
    (
        $query_name: ident $((context = $($context: tt)*))* {
            $($entity_name: ident(
                $graphql_struct: ident
                $(, filter = $filter_name: ident)*
                $(, limit = $limit: tt)*
                $(, offset = $offset: tt)*
                $(, order = $order: tt)*
            ),)*
        }
    ) => {
        #[derive(Debug)]
        pub struct $query_name<P>(::std::marker::PhantomData<P>);

        impl<P> Default for $query_name<P> {
            fn default() -> Self {
                $query_name(::std::marker::PhantomData)
            }
        }
        __wundergraph_expand_pg_loading_handler!{
            $query_name $((context = $($context)*))* {
                $($entity_name(
                    $graphql_struct
                        $(, filter = $filter_name)*
                        $(, limit = $limit)*
                        $(, offset = $offset)*
                        $(, order = $order)*
                ),)*
            }
        }

        __wundergraph_expand_sqlite_loading_handler!{
            $query_name $((context = $($context)*))* {
                $($entity_name(
                    $graphql_struct
                        $(, filter = $filter_name)*
                        $(, limit = $limit)*
                        $(, offset = $offset)*
                        $(, order = $order)*
                ),)*
            }
        }
    };
}
