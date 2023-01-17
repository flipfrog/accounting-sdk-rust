/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p> <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>http status codeが429となった場合、API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>x-ratelimit-limit:10 x-ratelimit-remaining:1 x-ratelimit-reset:2023-01-13T10:22:29+09:00 </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>  <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">x-ratelimit-limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  </br>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvoiceCreateParams {
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 請求日 (yyyy-mm-dd)
    #[serde(rename = "issue_date", skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    /// 取引先ID
    #[serde(rename = "partner_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<Option<i32>>,
    /// 取引先コード
    #[serde(rename = "partner_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_code: Option<Option<String>>,
    /// 請求書番号 (デフォルト: 自動採番されます)
    #[serde(rename = "invoice_number", skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// タイトル (デフォルト: 請求書)
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 期日 (yyyy-mm-dd)
    #[serde(rename = "due_date", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// 売上計上日
    #[serde(rename = "booking_date", skip_serializing_if = "Option::is_none")]
    pub booking_date: Option<String>,
    /// 概要
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 請求書ステータス<br> <ul>   <li>draft: 下書き (デフォルト)</li>   <li>(廃止予定) issue: 発行 (送付待ち (unsubmitted) と同じです。)</li>   <li>unsubmitted: 送付待ち</li>   <li>submitted: 送付済み</li> </ul> issue, unsubmitted, submitted は請求書承認ワークフローを利用している場合は指定できません。 
    #[serde(rename = "invoice_status", skip_serializing_if = "Option::is_none")]
    pub invoice_status: Option<InvoiceStatus>,
    /// 請求書に表示する取引先名
    #[serde(rename = "partner_display_name")]
    pub partner_display_name: String,
    /// 敬称（御中、様、(空白)の3つから選択）
    #[serde(rename = "partner_title", deserialize_with = "Option::deserialize")]
    pub partner_title: Option<String>,
    /// 取引先担当者名
    #[serde(rename = "partner_contact_info", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_contact_info: Option<Option<String>>,
    /// 取引先郵便番号 (デフォルトはpartner_idもしくはpartner_codeで指定された取引先設定情報が補完されます)
    #[serde(rename = "partner_zipcode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_zipcode: Option<Option<String>>,
    /// 取引先都道府県コード（0:北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄) (デフォルトはpartner_idもしくはpartner_codeで指定された取引先設定情報が補完されます)
    #[serde(rename = "partner_prefecture_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_prefecture_code: Option<Option<i32>>,
    /// 取引先市区町村・番地 (デフォルトはpartner_idもしくはpartner_codeで指定された取引先設定情報が補完されます)
    #[serde(rename = "partner_address1", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_address1: Option<Option<String>>,
    /// 取引先建物名・部屋番号など (デフォルトはpartner_idもしくはpartner_codeで指定された取引先設定情報が補完されます)
    #[serde(rename = "partner_address2", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_address2: Option<Option<String>>,
    /// 事業所名 (デフォルトは事業所設定情報が補完されます)
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 郵便番号 (デフォルトは事業所設定情報が補完されます)
    #[serde(rename = "company_zipcode", skip_serializing_if = "Option::is_none")]
    pub company_zipcode: Option<String>,
    /// 都道府県コード（0:北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄) (デフォルトは事業所設定情報が補完されます)
    #[serde(rename = "company_prefecture_code", skip_serializing_if = "Option::is_none")]
    pub company_prefecture_code: Option<i32>,
    /// 市区町村・番地 (デフォルトは事業所設定情報が補完されます)
    #[serde(rename = "company_address1", skip_serializing_if = "Option::is_none")]
    pub company_address1: Option<String>,
    /// 建物名・部屋番号など (デフォルトは事業所設定情報が補完されます)
    #[serde(rename = "company_address2", skip_serializing_if = "Option::is_none")]
    pub company_address2: Option<String>,
    /// 事業所担当者名 (デフォルトは請求書テンプレート情報が補完されます)
    #[serde(rename = "company_contact_info", skip_serializing_if = "Option::is_none")]
    pub company_contact_info: Option<String>,
    /// 支払方法 (振込: transfer, 引き落とし: direct_debit)
    #[serde(rename = "payment_type", skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    /// 支払口座
    #[serde(rename = "payment_bank_info", skip_serializing_if = "Option::is_none")]
    pub payment_bank_info: Option<String>,
    /// 振込専用口座の利用(利用しない: not_use(デフォルト), 利用する: use)
    #[serde(rename = "use_virtual_transfer_account", skip_serializing_if = "Option::is_none")]
    pub use_virtual_transfer_account: Option<UseVirtualTransferAccount>,
    /// メッセージ (デフォルト: 下記の通りご請求申し上げます。)
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 備考
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// 請求書レイアウト * `default_classic` - レイアウト１/クラシック (デフォルト)  * `standard_classic` - レイアウト２/クラシック  * `envelope_classic` - 封筒１/クラシック  * `carried_forward_standard_classic` - レイアウト３（繰越金額欄あり）/クラシック  * `carried_forward_envelope_classic` - 封筒２（繰越金額欄あり）/クラシック  * `default_modern` - レイアウト１/モダン  * `standard_modern` - レイアウト２/モダン  * `envelope_modern` - 封筒/モダン
    #[serde(rename = "invoice_layout", skip_serializing_if = "Option::is_none")]
    pub invoice_layout: Option<InvoiceLayout>,
    /// 請求書の消費税計算方法(inclusive: 内税表示, exclusive: 外税表示 (デフォルト))
    #[serde(rename = "tax_entry_method", skip_serializing_if = "Option::is_none")]
    pub tax_entry_method: Option<TaxEntryMethod>,
    /// 請求内容
    #[serde(rename = "invoice_contents", skip_serializing_if = "Option::is_none")]
    pub invoice_contents: Option<Vec<crate::models::InvoiceCreateParamsInvoiceContentsInner>>,
}

impl InvoiceCreateParams {
    pub fn new(company_id: i32, partner_display_name: String, partner_title: Option<String>) -> InvoiceCreateParams {
        InvoiceCreateParams {
            company_id,
            issue_date: None,
            partner_id: None,
            partner_code: None,
            invoice_number: None,
            title: None,
            due_date: None,
            booking_date: None,
            description: None,
            invoice_status: None,
            partner_display_name,
            partner_title,
            partner_contact_info: None,
            partner_zipcode: None,
            partner_prefecture_code: None,
            partner_address1: None,
            partner_address2: None,
            company_name: None,
            company_zipcode: None,
            company_prefecture_code: None,
            company_address1: None,
            company_address2: None,
            company_contact_info: None,
            payment_type: None,
            payment_bank_info: None,
            use_virtual_transfer_account: None,
            message: None,
            notes: None,
            invoice_layout: None,
            tax_entry_method: None,
            invoice_contents: None,
        }
    }
}

/// 請求書ステータス<br> <ul>   <li>draft: 下書き (デフォルト)</li>   <li>(廃止予定) issue: 発行 (送付待ち (unsubmitted) と同じです。)</li>   <li>unsubmitted: 送付待ち</li>   <li>submitted: 送付済み</li> </ul> issue, unsubmitted, submitted は請求書承認ワークフローを利用している場合は指定できません。 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvoiceStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "unsubmitted")]
    Unsubmitted,
    #[serde(rename = "submitted")]
    Submitted,
}

impl Default for InvoiceStatus {
    fn default() -> InvoiceStatus {
        Self::Draft
    }
}
/// 支払方法 (振込: transfer, 引き落とし: direct_debit)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentType {
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "direct_debit")]
    DirectDebit,
}

impl Default for PaymentType {
    fn default() -> PaymentType {
        Self::Transfer
    }
}
/// 振込専用口座の利用(利用しない: not_use(デフォルト), 利用する: use)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UseVirtualTransferAccount {
    #[serde(rename = "not_use")]
    NotUse,
    #[serde(rename = "use")]
    Use,
}

impl Default for UseVirtualTransferAccount {
    fn default() -> UseVirtualTransferAccount {
        Self::NotUse
    }
}
/// 請求書レイアウト * `default_classic` - レイアウト１/クラシック (デフォルト)  * `standard_classic` - レイアウト２/クラシック  * `envelope_classic` - 封筒１/クラシック  * `carried_forward_standard_classic` - レイアウト３（繰越金額欄あり）/クラシック  * `carried_forward_envelope_classic` - 封筒２（繰越金額欄あり）/クラシック  * `default_modern` - レイアウト１/モダン  * `standard_modern` - レイアウト２/モダン  * `envelope_modern` - 封筒/モダン
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvoiceLayout {
    #[serde(rename = "default_classic")]
    DefaultClassic,
    #[serde(rename = "standard_classic")]
    StandardClassic,
    #[serde(rename = "envelope_classic")]
    EnvelopeClassic,
    #[serde(rename = "carried_forward_standard_classic")]
    CarriedForwardStandardClassic,
    #[serde(rename = "carried_forward_envelope_classic")]
    CarriedForwardEnvelopeClassic,
    #[serde(rename = "default_modern")]
    DefaultModern,
    #[serde(rename = "standard_modern")]
    StandardModern,
    #[serde(rename = "envelope_modern")]
    EnvelopeModern,
}

impl Default for InvoiceLayout {
    fn default() -> InvoiceLayout {
        Self::DefaultClassic
    }
}
/// 請求書の消費税計算方法(inclusive: 内税表示, exclusive: 外税表示 (デフォルト))
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxEntryMethod {
    #[serde(rename = "inclusive")]
    Inclusive,
    #[serde(rename = "exclusive")]
    Exclusive,
}

impl Default for TaxEntryMethod {
    fn default() -> TaxEntryMethod {
        Self::Inclusive
    }
}

