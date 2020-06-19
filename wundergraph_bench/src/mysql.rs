use crate::api::{FilmActor, NewFilmActor};
use diesel::prelude::*;
use juniper::{ExecutionResult, Executor, Selection, Value};
use wundergraph::query_builder::mutations::{HandleBatchInsert, HandleInsert};
use wundergraph::query_builder::selection::LoadingHandler;
use wundergraph::{QueryModifier, WundergraphContext};

impl<Ctx> HandleInsert<FilmActor, NewFilmActor, diesel::mysql::Mysql, Ctx>
    for crate::api::film_actor::table
where
    Ctx: WundergraphContext + QueryModifier<FilmActor, diesel::mysql::Mysql> + 'static,
    Ctx::Connection: Connection<Backend = diesel::mysql::Mysql>,
{
    fn handle_insert(
        selection: Option<&'_ [Selection<'_, wundergraph::scalar::WundergraphScalarValue>]>,
        executor: &Executor<'_, Ctx, wundergraph::scalar::WundergraphScalarValue>,
        insertable: NewFilmActor,
    ) -> ExecutionResult<wundergraph::scalar::WundergraphScalarValue> {
        // let ctx = executor.context();
        // let conn = ctx.get_connection();
        // let look_ahead = executor.look_ahead();

        // conn.transaction(|| {
        //     diesel::insert_into(crate::api::film_actor::table)
        //         .values((
        //             crate::api::film_actor::actor_id.eq(insertable.actor_id),
        //             crate::api::film_actor::film_id.eq(insertable.film_id),
        //         ))
        //         .execute(conn)?;

        //     let query = <FilmActor as LoadingHandler<diesel::mysql::Mysql, Ctx>>::build_query(
        //         &[],
        //         &look_ahead,
        //     )?
        //     .filter(
        //         crate::api::film_actor::actor_id
        //             .eq(insertable.actor_id)
        //             .and(crate::api::film_actor::film_id.eq(insertable.film_id)),
        //     )
        //     .limit(1);

        //     let items = FilmActor::load(&look_ahead, selection, executor, query)?;
        //     Ok(items.into_iter().next().unwrap_or(Value::Null))
        // })
        unimplemented!()
    }
}

impl<Ctx> HandleBatchInsert<FilmActor, NewFilmActor, diesel::mysql::Mysql, Ctx>
    for crate::api::film_actor::table
where
    Ctx: WundergraphContext + QueryModifier<FilmActor, diesel::mysql::Mysql> + 'static,
    Ctx::Connection: Connection<Backend = diesel::mysql::Mysql>,
{
    fn handle_batch_insert(
        selection: Option<&'_ [Selection<'_, wundergraph::scalar::WundergraphScalarValue>]>,
        executor: &Executor<'_, Ctx, wundergraph::scalar::WundergraphScalarValue>,
        insertable: Vec<NewFilmActor>,
    ) -> ExecutionResult<wundergraph::scalar::WundergraphScalarValue> {
        // let ctx = executor.context();
        // let conn = ctx.get_connection();
        // let look_ahead = executor.look_ahead();

        // conn.transaction(|| {
        //     {
        //         let insert_values = insertable
        //             .iter()
        //             .map(|NewFilmActor { actor_id, film_id }| {
        //                 (
        //                     crate::api::film_actor::actor_id.eq(actor_id),
        //                     crate::api::film_actor::film_id.eq(film_id),
        //                 )
        //             })
        //             .collect::<Vec<_>>();
        //         diesel::insert_into(crate::api::film_actor::table)
        //             .values(insert_values)
        //             .execute(conn)?;
        //     }

        //     let mut query = <FilmActor as LoadingHandler<diesel::mysql::Mysql, Ctx>>::build_query(
        //         &[],
        //         &look_ahead,
        //     )?;

        //     for NewFilmActor { actor_id, film_id } in insertable {
        //         query = query.or_filter(
        //             crate::api::film_actor::actor_id
        //                 .eq(actor_id)
        //                 .and(crate::api::film_actor::film_id.eq(film_id)),
        //         )
        //     }

        //     let items = FilmActor::load(&look_ahead, selection, executor, query)?;
        //     Ok(Value::list(items))
        // })
        unimplemented!()
    }
}
