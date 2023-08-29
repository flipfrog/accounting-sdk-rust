/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">お知らせ</h2>  <p> <b>インボイス制度に伴い、freee会計の帳票機能がfreee請求書に移行します。これに伴い、2023年10月にfreee会計の「請求書の作成、見積書の作成」エンドポイントは廃止、freee請求書APIに移行する予定です。詳細は<a href=\"https://developer.freee.co.jp/news/6369\" target=\"_blank\"> freee会計 APIの仕様変更（インボイス制度対応）について</a>をご確認ください。</b> </p>  <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。<a href=\"https://developer.freee.co.jp/reference/#%e8%aa%8d%e8%a8%bc\" target=\"_blank\">詳細はリファレンスの認証に関する記載を参照してください。</a></p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> <li>エラーメッセージの変更</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p> <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>http status codeが429となった場合、API使用ステータスはレスポンスヘッダに付与されます。</p> <pre><code>x-ratelimit-limit:10 x-ratelimit-remaining:1 x-ratelimit-reset:2023-01-13T10:22:29+09:00 </code></pre>  <br> 各ヘッダの意味は次のとおりです。</p>  <table border=\"1\">   <tbody>     <tr>       <th style=\"padding: 10px\"><strong>ヘッダ名</strong></th>       <th style=\"padding: 10px\"><strong>説明</strong></th>     </tr>     <tr><td style=\"padding: 10px\">x-ratelimit-limit</td><td style=\"padding: 10px\">使用回数の上限</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-remaining</td><td style=\"padding: 10px\">残り使用回数</td></tr>     <tr><td style=\"padding: 10px\">x-ratelimit-reset</td><td style=\"padding: 10px\">使用回数がリセットされる時刻</td></tr>   </tbody> </table>  </br>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartnersResponsePartnersInner {
    /// 取引先ID
    #[serde(rename = "id")]
    pub id: i32,
    /// 取引先コード
    #[serde(rename = "code", deserialize_with = "Option::deserialize")]
    pub code: Option<String>,
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 取引先名
    #[serde(rename = "name")]
    pub name: String,
    /// 更新日 (yyyy-mm-dd)
    #[serde(rename = "update_date")]
    pub update_date: String,
    /// 取引先の使用設定（true: 使用する、false: 使用しない） <br> <ul>   <li>     本APIでpartnerを作成した場合はtrueになります。   </li>   <li>     falseにする場合はWeb画面から変更できます。   </li>   <li>     trueの場合、Web画面での取引登録時などに入力候補として表示されます。   </li>   <li>     falseの場合、取引先自体は削除せず、Web画面での取引登録時などに入力候補として表示されません。ただし取引（収入・支出）の作成APIなどでfalseの取引先をパラメータに指定すれば、取引などにfalseの取引先を設定できます。   </li> </ul>
    #[serde(rename = "available")]
    pub available: bool,
    /// ショートカット1 (255文字以内)
    #[serde(rename = "shortcut1", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shortcut1: Option<Option<String>>,
    /// ショートカット2 (255文字以内)
    #[serde(rename = "shortcut2", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shortcut2: Option<Option<String>>,
    /// 事業所種別（null: 未設定、1: 法人、2: 個人）
    #[serde(rename = "org_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub org_code: Option<Option<i32>>,
    /// 地域（JP: 国内、ZZ:国外）
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// 正式名称（255文字以内）
    #[serde(rename = "long_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub long_name: Option<Option<String>>,
    /// カナ名称（255文字以内）
    #[serde(rename = "name_kana", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<Option<String>>,
    /// 敬称（御中、様、(空白)の3つから選択）
    #[serde(rename = "default_title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_title: Option<Option<String>>,
    /// 電話番号
    #[serde(rename = "phone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// 担当者 氏名
    #[serde(rename = "contact_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<Option<String>>,
    /// 担当者 メールアドレス
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// 振込元口座ID（一括振込ファイル用）:（未設定の場合は、nullです。）
    #[serde(rename = "payer_walletable_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payer_walletable_id: Option<Option<i32>>,
    /// 振込手数料負担（一括振込ファイル用）: (振込元(当方): payer, 振込先(先方): payee)
    #[serde(rename = "transfer_fee_handling_side", skip_serializing_if = "Option::is_none")]
    pub transfer_fee_handling_side: Option<TransferFeeHandlingSide>,
    /// インボイス制度適格請求書発行事業者（true: 対象事業者、false: 非対象事業者） <a target=\"_blank\" href=\"https://www.invoice-kohyo.nta.go.jp/index.html\">国税庁インボイス制度適格請求書発行事業者公表サイト</a> 
    #[serde(rename = "qualified_invoice_issuer", skip_serializing_if = "Option::is_none")]
    pub qualified_invoice_issuer: Option<bool>,
    /// インボイス制度適格請求書発行事業者登録番号 - 先頭T数字13桁の固定14桁の文字列 <a target=\"_blank\" href=\"https://www.invoice-kohyo.nta.go.jp/index.html\">国税庁インボイス制度適格請求書発行事業者公表サイト</a> 
    #[serde(rename = "invoice_registration_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub invoice_registration_number: Option<Option<String>>,
    #[serde(rename = "address_attributes", skip_serializing_if = "Option::is_none")]
    pub address_attributes: Option<Box<crate::models::PartnersResponsePartnersInnerAddressAttributes>>,
    #[serde(rename = "partner_doc_setting_attributes", skip_serializing_if = "Option::is_none")]
    pub partner_doc_setting_attributes: Option<Box<crate::models::PartnerCreateParamsPartnerDocSettingAttributes>>,
    #[serde(rename = "partner_bank_account_attributes", skip_serializing_if = "Option::is_none")]
    pub partner_bank_account_attributes: Option<Box<crate::models::PartnersResponsePartnersInnerPartnerBankAccountAttributes>>,
}

impl PartnersResponsePartnersInner {
    pub fn new(id: i32, code: Option<String>, company_id: i32, name: String, update_date: String, available: bool) -> PartnersResponsePartnersInner {
        PartnersResponsePartnersInner {
            id,
            code,
            company_id,
            name,
            update_date,
            available,
            shortcut1: None,
            shortcut2: None,
            org_code: None,
            country_code: None,
            long_name: None,
            name_kana: None,
            default_title: None,
            phone: None,
            contact_name: None,
            email: None,
            payer_walletable_id: None,
            transfer_fee_handling_side: None,
            qualified_invoice_issuer: None,
            invoice_registration_number: None,
            address_attributes: None,
            partner_doc_setting_attributes: None,
            partner_bank_account_attributes: None,
        }
    }
}

/// 振込手数料負担（一括振込ファイル用）: (振込元(当方): payer, 振込先(先方): payee)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferFeeHandlingSide {
    #[serde(rename = "payer")]
    Payer,
    #[serde(rename = "payee")]
    Payee,
}

impl Default for TransferFeeHandlingSide {
    fn default() -> TransferFeeHandlingSide {
        Self::Payer
    }
}

