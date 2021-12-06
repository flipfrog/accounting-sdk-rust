/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。詳細は<a href=\"https://developer.freee.co.jp/docs\" target=\"_blank\">ドキュメントの認証</a>パートを参照してください。</p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p>  <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>レスポンスボディのmetaプロパティに以下を含めます。</p>  <ul>   <li>設定されている上限値</li>   <li>上限に達するまでの使用可能回数</li>   <li>（上限値に達した場合）使用回数がリセットされる時刻</li> </ul>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotationResponseQuotation {
    /// 市区町村・番地
    #[serde(rename = "company_address1", skip_serializing_if = "Option::is_none")]
    pub company_address1: Option<String>,
    /// 建物名・部屋番号など
    #[serde(rename = "company_address2", skip_serializing_if = "Option::is_none")]
    pub company_address2: Option<String>,
    /// 事業所担当者名
    #[serde(rename = "company_contact_info", skip_serializing_if = "Option::is_none")]
    pub company_contact_info: Option<String>,
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 事業所名
    #[serde(rename = "company_name")]
    pub company_name: String,
    /// 都道府県コード（-1: 設定しない、0:北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄
    #[serde(rename = "company_prefecture_code", skip_serializing_if = "Option::is_none")]
    pub company_prefecture_code: Option<i32>,
    /// 都道府県
    #[serde(rename = "company_prefecture_name", skip_serializing_if = "Option::is_none")]
    pub company_prefecture_name: Option<String>,
    /// 郵便番号
    #[serde(rename = "company_zipcode", skip_serializing_if = "Option::is_none")]
    pub company_zipcode: Option<String>,
    /// 概要
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 見積書ID
    #[serde(rename = "id")]
    pub id: i32,
    /// 見積日 (yyyy-mm-dd)
    #[serde(rename = "issue_date")]
    pub issue_date: String,
    /// メール送信日時(最新)
    #[serde(rename = "mail_sent_at", skip_serializing_if = "Option::is_none")]
    pub mail_sent_at: Option<String>,
    /// メッセージ
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 備考
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// 市区町村・番地
    #[serde(rename = "partner_address1", skip_serializing_if = "Option::is_none")]
    pub partner_address1: Option<String>,
    /// 建物名・部屋番号など
    #[serde(rename = "partner_address2", skip_serializing_if = "Option::is_none")]
    pub partner_address2: Option<String>,
    /// 取引先コード
    #[serde(rename = "partner_code", skip_serializing_if = "Option::is_none")]
    pub partner_code: Option<String>,
    /// 取引先担当者名
    #[serde(rename = "partner_contact_info", skip_serializing_if = "Option::is_none")]
    pub partner_contact_info: Option<String>,
    /// 見積書に表示する取引先名
    #[serde(rename = "partner_display_name", skip_serializing_if = "Option::is_none")]
    pub partner_display_name: Option<String>,
    /// 取引先ID
    #[serde(rename = "partner_id")]
    pub partner_id: Option<i32>,
    /// 取引先名
    #[serde(rename = "partner_name", skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    /// 都道府県コード（-1: 設定しない、0:北海道、1:青森、2:岩手、3:宮城、4:秋田、5:山形、6:福島、7:茨城、8:栃木、9:群馬、10:埼玉、11:千葉、12:東京、13:神奈川、14:新潟、15:富山、16:石川、17:福井、18:山梨、19:長野、20:岐阜、21:静岡、22:愛知、23:三重、24:滋賀、25:京都、26:大阪、27:兵庫、28:奈良、29:和歌山、30:鳥取、31:島根、32:岡山、33:広島、34:山口、35:徳島、36:香川、37:愛媛、38:高知、39:福岡、40:佐賀、41:長崎、42:熊本、43:大分、44:宮崎、45:鹿児島、46:沖縄
    #[serde(rename = "partner_prefecture_code", skip_serializing_if = "Option::is_none")]
    pub partner_prefecture_code: Option<i32>,
    /// 都道府県
    #[serde(rename = "partner_prefecture_name", skip_serializing_if = "Option::is_none")]
    pub partner_prefecture_name: Option<String>,
    /// 敬称（御中、様、(空白)の3つから選択）
    #[serde(rename = "partner_title")]
    pub partner_title: Option<String>,
    /// 郵便番号
    #[serde(rename = "partner_zipcode", skip_serializing_if = "Option::is_none")]
    pub partner_zipcode: Option<String>,
    /// 見積内容
    #[serde(rename = "quotation_contents", skip_serializing_if = "Option::is_none")]
    pub quotation_contents: Option<Vec<crate::models::QuotationIndexResponseQuotationContents>>,
    /// 見積書レイアウト * `default_classic` - レイアウト１/クラシック (デフォルト)  * `standard_classic` - レイアウト２/クラシック  * `envelope_classic` - 封筒１/クラシック  * `default_modern` - レイアウト１/モダン  * `standard_modern` - レイアウト２/モダン  * `envelope_modern` - 封筒/モダン
    #[serde(rename = "quotation_layout")]
    pub quotation_layout: QuotationLayout,
    /// 見積書番号
    #[serde(rename = "quotation_number")]
    pub quotation_number: String,
    /// 見積書ステータス  (unsubmitted: 送付待ち, submitted: 送付済み, all: 全て)
    #[serde(rename = "quotation_status")]
    pub quotation_status: QuotationStatus,
    /// 関連する請求書ID<br> 下記で作成したものが該当します。  <a href=\"https://support.freee.co.jp/hc/ja/articles/203318410#1-2\" target=\"_blank\">見積書・納品書を納品書・請求書に変換する</a><br> <a href=\"https://support.freee.co.jp/hc/ja/articles/209076226\" target=\"_blank\">複数の見積書・納品書から合算請求書を作成する</a><br> 
    #[serde(rename = "related_invoice_id", skip_serializing_if = "Option::is_none")]
    pub related_invoice_id: Option<i32>,
    /// 関連する見積書ID(配列)<br> 下記で作成したものが該当します。  <a href=\"https://support.freee.co.jp/hc/ja/articles/203318410#1-2\" target=\"_blank\">見積書・納品書を納品書・請求書に変換する</a><br> <a href=\"https://support.freee.co.jp/hc/ja/articles/209076226\" target=\"_blank\">複数の見積書・納品書から合算請求書を作成する</a><br> 
    #[serde(rename = "related_quotation_ids", skip_serializing_if = "Option::is_none")]
    pub related_quotation_ids: Option<Vec<i32>>,
    /// 小計
    #[serde(rename = "sub_total", skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<i32>,
    /// 見積書の消費税計算方法(inclusive: 内税, exclusive: 外税)
    #[serde(rename = "tax_entry_method")]
    pub tax_entry_method: TaxEntryMethod,
    /// タイトル
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 合計金額
    #[serde(rename = "total_amount")]
    pub total_amount: i32,
    #[serde(rename = "total_amount_per_vat_rate")]
    pub total_amount_per_vat_rate: Box<crate::models::InvoiceIndexResponseTotalAmountPerVatRate>,
    /// 消費税
    #[serde(rename = "total_vat", skip_serializing_if = "Option::is_none")]
    pub total_vat: Option<i32>,
    /// Web共有取引先確認日時(最新)
    #[serde(rename = "web_confirmed_at", skip_serializing_if = "Option::is_none")]
    pub web_confirmed_at: Option<String>,
    /// Web共有ダウンロード日時(最新)
    #[serde(rename = "web_downloaded_at", skip_serializing_if = "Option::is_none")]
    pub web_downloaded_at: Option<String>,
    /// Web共有日時(最新)
    #[serde(rename = "web_published_at", skip_serializing_if = "Option::is_none")]
    pub web_published_at: Option<String>,
}

impl QuotationResponseQuotation {
    pub fn new(company_id: i32, company_name: String, id: i32, issue_date: String, partner_id: Option<i32>, partner_title: Option<String>, quotation_layout: QuotationLayout, quotation_number: String, quotation_status: QuotationStatus, tax_entry_method: TaxEntryMethod, total_amount: i32, total_amount_per_vat_rate: crate::models::InvoiceIndexResponseTotalAmountPerVatRate) -> QuotationResponseQuotation {
        QuotationResponseQuotation {
            company_address1: None,
            company_address2: None,
            company_contact_info: None,
            company_id,
            company_name,
            company_prefecture_code: None,
            company_prefecture_name: None,
            company_zipcode: None,
            description: None,
            id,
            issue_date,
            mail_sent_at: None,
            message: None,
            notes: None,
            partner_address1: None,
            partner_address2: None,
            partner_code: None,
            partner_contact_info: None,
            partner_display_name: None,
            partner_id,
            partner_name: None,
            partner_prefecture_code: None,
            partner_prefecture_name: None,
            partner_title,
            partner_zipcode: None,
            quotation_contents: None,
            quotation_layout,
            quotation_number,
            quotation_status,
            related_invoice_id: None,
            related_quotation_ids: None,
            sub_total: None,
            tax_entry_method,
            title: None,
            total_amount,
            total_amount_per_vat_rate: Box::new(total_amount_per_vat_rate),
            total_vat: None,
            web_confirmed_at: None,
            web_downloaded_at: None,
            web_published_at: None,
        }
    }
}

/// 見積書レイアウト * `default_classic` - レイアウト１/クラシック (デフォルト)  * `standard_classic` - レイアウト２/クラシック  * `envelope_classic` - 封筒１/クラシック  * `default_modern` - レイアウト１/モダン  * `standard_modern` - レイアウト２/モダン  * `envelope_modern` - 封筒/モダン
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QuotationLayout {
    #[serde(rename = "default_classic")]
    DefaultClassic,
    #[serde(rename = "standard_classic")]
    StandardClassic,
    #[serde(rename = "envelope_classic")]
    EnvelopeClassic,
    #[serde(rename = "default_modern")]
    DefaultModern,
    #[serde(rename = "standard_modern")]
    StandardModern,
    #[serde(rename = "envelope_modern")]
    EnvelopeModern,
}
/// 見積書ステータス  (unsubmitted: 送付待ち, submitted: 送付済み, all: 全て)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QuotationStatus {
    #[serde(rename = "unsubmitted")]
    Unsubmitted,
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "all")]
    All,
}
/// 見積書の消費税計算方法(inclusive: 内税, exclusive: 外税)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxEntryMethod {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "inclusive")]
    Inclusive,
    #[serde(rename = "exclusive")]
    Exclusive,
}

