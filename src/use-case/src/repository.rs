use crate::Result;
use fridgers_backend_domain::compartment::Compartment;
use fridgers_backend_domain::fridge::Fridge;
use fridgers_backend_domain::item::Item;
use fridgers_backend_domain::user::User;

/// リポジトリのインターフェイス
pub trait Repository: Send + Sync {
    /// ユーザーを保存する
    fn save_user(&self, user: &User) -> impl Future<Output = Result<()>> + Send;

    /// ユーザーIDでユーザーを検索する
    fn find_user_by_id(&self, id: &str) -> impl Future<Output = Result<Option<User>>> + Send;

    /// ユーザーを削除する
    fn delete_user(&self, id: &str) -> impl Future<Output = Result<()>> + Send;

    /// メールアドレスでユーザーを検索する
    fn find_user_by_email(&self, email: &str) -> impl Future<Output = Result<Option<User>>> + Send;

    /// 冷蔵庫を保存する
    fn save_fridge(&self, fridge: &Fridge) -> impl Future<Output = Result<()>> + Send;

    /// 冷蔵庫IDで冷蔵庫を検索する
    fn find_fridge_by_id(&self, id: &str) -> impl Future<Output = Result<Option<Fridge>>> + Send;

    /// 冷蔵庫を削除する
    fn delete_fridge(&self, id: &str) -> impl Future<Output = Result<()>> + Send;

    /// コンパートメントを保存する
    fn save_compartment(
        &self,
        compartment: &Compartment,
    ) -> impl Future<Output = Result<()>> + Send;

    /// コンパートメントIDでコンパートメントを検索する
    fn find_compartment_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Compartment>>> + Send;

    /// 冷蔵庫IDに紐づくコンパートメント一覧を取得する
    fn find_compartments_by_fridge_id(
        &self,
        fridge_id: &str,
    ) -> impl Future<Output = Result<Vec<Compartment>>> + Send;

    /// コンパートメントを更新する
    fn update_compartment(
        &self,
        compartment: &Compartment,
    ) -> impl Future<Output = Result<()>> + Send;

    /// コンパートメントを削除する
    fn delete_compartment(&self, id: &str) -> impl Future<Output = Result<()>> + Send;

    /// アイテムを保存する
    fn save_item(&self, item: &Item) -> impl Future<Output = Result<()>> + Send;

    /// アイテムIDでアイテムを検索する
    fn find_item_by_id(&self, id: &str) -> impl Future<Output = Result<Option<Item>>> + Send;

    /// コンパートメントIDに紐づくアイテム一覧を取得する
    fn find_items_by_compartment_id(
        &self,
        compartment_id: &str,
    ) -> impl Future<Output = Result<Vec<Item>>> + Send;

    /// アイテムを更新する
    fn update_item(&self, item: &Item) -> impl Future<Output = Result<()>> + Send;

    /// アイテムを削除する
    fn delete_item(&self, id: &str) -> impl Future<Output = Result<()>> + Send;
}
