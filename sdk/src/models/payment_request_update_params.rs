/*
 * freee API
 *
 *  <h1 id=\"freee_api\">freee API</h1> <hr /> <h2 id=\"start_guide\">スタートガイド</h2>  <p>freee API開発がはじめての方は<a href=\"https://developer.freee.co.jp/getting-started\">freee API スタートガイド</a>を参照してください。</p>  <hr /> <h2 id=\"specification\">仕様</h2>  <h3 id=\"api_endpoint\">APIエンドポイント</h3>  <p>https://api.freee.co.jp/ (httpsのみ)</p>  <h3 id=\"about_authorize\">認証について</h3> <p>OAuth2.0を利用します。詳細は<a href=\"https://developer.freee.co.jp/docs\" target=\"_blank\">ドキュメントの認証</a>パートを参照してください。</p>  <h3 id=\"data_format\">データフォーマット</h3>  <p>リクエスト、レスポンスともにJSON形式をサポートしていますが、詳細は、API毎の説明欄（application/jsonなど）を確認してください。</p>  <h3 id=\"compatibility\">後方互換性ありの変更</h3>  <p>freeeでは、APIを改善していくために以下のような変更は後方互換性ありとして通知なく変更を入れることがあります。アプリケーション実装者は以下を踏まえて開発を行ってください。</p>  <ul> <li>新しいAPIリソース・エンドポイントの追加</li> <li>既存のAPIに対して必須ではない新しいリクエストパラメータの追加</li> <li>既存のAPIレスポンスに対する新しいプロパティの追加</li> <li>既存のAPIレスポンスに対するプロパティの順番の入れ変え</li> <li>keyとなっているidやcodeの長さの変更（長くする）</li> </ul>  <h3 id=\"common_response_header\">共通レスポンスヘッダー</h3>  <p>すべてのAPIのレスポンスには以下のHTTPヘッダーが含まれます。</p>  <ul> <li> <p>X-Freee-Request-ID</p> <ul> <li>各リクエスト毎に発行されるID</li> </ul> </li> </ul>  <h3 id=\"common_error_response\">共通エラーレスポンス</h3>  <ul> <li> <p>ステータスコードはレスポンス内のJSONに含まれる他、HTTPヘッダにも含まれる</p> </li> <li> <p>一部のエラーレスポンスにはエラーコードが含まれます。<br>詳細は、<a href=\"https://developer.freee.co.jp/tips/faq/40x-checkpoint\">HTTPステータスコード400台エラー時のチェックポイント</a>を参照してください</p> </li> <p>type</p>  <ul> <li>status : HTTPステータスコードの説明</li>  <li>validation : エラーの詳細の説明（開発者向け）</li> </ul> </li> </ul>  <p>レスポンスの例</p>  <pre><code>  {     &quot;status_code&quot; : 400,     &quot;errors&quot; : [       {         &quot;type&quot; : &quot;status&quot;,         &quot;messages&quot; : [&quot;不正なリクエストです。&quot;]       },       {         &quot;type&quot; : &quot;validation&quot;,         &quot;messages&quot; : [&quot;Date は不正な日付フォーマットです。入力例：2019-12-17&quot;]       }     ]   }</code></pre>  </br>  <h3 id=\"api_rate_limit\">API使用制限</h3>    <p>freeeは一定期間に過度のアクセスを検知した場合、APIアクセスをコントロールする場合があります。</p>   <p>その際のhttp status codeは403となります。制限がかかってから10分程度が過ぎると再度使用することができるようになります。</p>  <h4 id=\"reports_api_endpoint\">/reportsと/receipts/{id}/downloadエンドポイント</h4>  <p>freeeはエンドポイント毎に一定頻度以上のアクセスを検知した場合、APIアクセスをコントロールする場合があります。その際のhttp status codeは429（too many requests）となります。</p>  <ul>   <li>/reports:1秒に10回まで</li>   <li>/receipts/{id}/download:1秒に3回まで</li> </ul>  <p>レスポンスボディのmetaプロパティに以下を含めます。</p>  <ul>   <li>設定されている上限値</li>   <li>上限に達するまでの使用可能回数</li>   <li>（上限値に達した場合）使用回数がリセットされる時刻</li> </ul>  <h3 id=\"plan_api_rate_limit\">プラン別のAPI Rate Limit</h3>   <table border=\"1\">     <tbody>       <tr>         <th style=\"padding: 10px\"><strong>freee会計プラン名</strong></th>         <th style=\"padding: 10px\"><strong>事業所とアプリケーション毎に1日でのAPIコール数</strong></th>       </tr>       <tr>         <td style=\"padding: 10px\">エンタープライズ</td>         <td style=\"padding: 10px\">10,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">プロフェッショナル</td>         <td style=\"padding: 10px\">5,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ベーシック</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">ミニマム</td>         <td style=\"padding: 10px\">3,000</td>       </tr>       <tr>         <td style=\"padding: 10px\">上記以外</td>         <td style=\"padding: 10px\">3,000</td>       </tr>     </tbody>   </table>  <h3 id=\"webhook\">Webhookについて</h3>  <p>詳細は<a href=\"https://developer.freee.co.jp/docs/accounting/webhook\" target=\"_blank\">会計Webhook概要</a>を参照してください。</p>  <hr /> <h2 id=\"contact\">連絡先</h2>  <p>ご不明点、ご要望等は <a href=\"https://support.freee.co.jp/hc/ja/requests/new\">freee サポートデスクへのお問い合わせフォーム</a> からご連絡ください。</p> <hr />&copy; Since 2013 freee K.K.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentRequestUpdateParams {
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 申請タイトル<br> 申請者が、下書き状態もしくは差戻し状態の支払依頼に対して指定する場合のみ有効 
    #[serde(rename = "title")]
    pub title: String,
    /// 申請日 (yyyy-mm-dd)<br> 指定しない場合は当日の日付が登録されます。<br> 申請者が、下書き状態もしくは差戻し状態の支払依頼に対して指定する場合のみ有効 
    #[serde(rename = "application_date", skip_serializing_if = "Option::is_none")]
    pub application_date: Option<String>,
    /// 備考
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 支払依頼の項目行一覧（配列）
    #[serde(rename = "payment_request_lines")]
    pub payment_request_lines: Vec<crate::models::PaymentRequestUpdateParamsPaymentRequestLinesInner>,
    /// 承認者のユーザーID<br> 「承認者を指定」の経路を申請経路として使用する場合に指定してください。<br> 指定する承認者のユーザーIDは、申請経路APIを利用して取得してください。 
    #[serde(rename = "approver_id", skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<i32>,
    /// 申請経路ID<br> 指定する申請経路IDは、申請経路APIを利用して取得してください。 
    #[serde(rename = "approval_flow_route_id")]
    pub approval_flow_route_id: i32,
    /// 支払依頼のステータス<br> falseを指定した時は申請中（in_progress）で支払依頼を更新します。<br> trueを指定した時は下書き（draft）で支払依頼を更新します。 
    #[serde(rename = "draft")]
    pub draft: bool,
    /// 請求書番号（255文字以内）
    #[serde(rename = "document_code", skip_serializing_if = "Option::is_none")]
    pub document_code: Option<String>,
    /// 証憑ファイルID（ファイルボックスのファイルID）（配列）
    #[serde(rename = "receipt_ids", skip_serializing_if = "Option::is_none")]
    pub receipt_ids: Option<Vec<i32>>,
    /// 発生日 (yyyy-mm-dd)
    #[serde(rename = "issue_date")]
    pub issue_date: String,
    /// 支払期限 (yyyy-mm-dd)
    #[serde(rename = "payment_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<Option<String>>,
    /// '支払方法(none: 指定なし, domestic_bank_transfer: 国内振込, abroad_bank_transfer: 国外振込, account_transfer: 口座振替, credit_card: クレジットカード)'<br> 'デフォルトは none: 指定なし です。' 
    #[serde(rename = "payment_method", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    /// 支払先の取引先ID
    #[serde(rename = "partner_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<Option<i32>>,
    /// 支払先の取引先コード<br> 支払先の取引先ID指定時には無効 
    #[serde(rename = "partner_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub partner_code: Option<Option<String>>,
    /// 銀行コード（半角数字1桁〜4桁）<br> 支払先指定時には無効 
    #[serde(rename = "bank_code", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// 銀行名（255文字以内）<br> 支払先指定時には無効 
    #[serde(rename = "bank_name", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// 銀行名（カナ）（15文字以内）<br> 支払先指定時には無効 
    #[serde(rename = "bank_name_kana", skip_serializing_if = "Option::is_none")]
    pub bank_name_kana: Option<String>,
    /// 支店番号（半角数字1桁〜3桁）<br> 支払先指定時には無効 
    #[serde(rename = "branch_code", skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// 支店名（255文字以内）<br> 支払先指定時には無効 
    #[serde(rename = "branch_name", skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// 支店名（カナ）（15文字以内）<br> 指定可能な文字は、英数・カナ・丸括弧・ハイフン・スペースのみです。<br> 支払先指定時には無効 
    #[serde(rename = "branch_kana", skip_serializing_if = "Option::is_none")]
    pub branch_kana: Option<String>,
    /// 受取人名（カナ）（48文字以内）<br> 支払先指定時には無効 
    #[serde(rename = "account_name", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// 口座番号（半角数字1桁〜7桁）<br> 支払先指定時には無効 
    #[serde(rename = "account_number", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// '口座種別(ordinary: 普通、checking: 当座、earmarked: 納税準備預金、savings: 貯蓄、other: その他)'<br> '支払先指定時には無効'<br> 'デフォルトは ordinary: 普通 です' 
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
}

impl PaymentRequestUpdateParams {
    pub fn new(company_id: i32, title: String, payment_request_lines: Vec<crate::models::PaymentRequestUpdateParamsPaymentRequestLinesInner>, approval_flow_route_id: i32, draft: bool, issue_date: String) -> PaymentRequestUpdateParams {
        PaymentRequestUpdateParams {
            company_id,
            title,
            application_date: None,
            description: None,
            payment_request_lines,
            approver_id: None,
            approval_flow_route_id,
            draft,
            document_code: None,
            receipt_ids: None,
            issue_date,
            payment_date: None,
            payment_method: None,
            partner_id: None,
            partner_code: None,
            bank_code: None,
            bank_name: None,
            bank_name_kana: None,
            branch_code: None,
            branch_name: None,
            branch_kana: None,
            account_name: None,
            account_number: None,
            account_type: None,
        }
    }
}

/// '支払方法(none: 指定なし, domestic_bank_transfer: 国内振込, abroad_bank_transfer: 国外振込, account_transfer: 口座振替, credit_card: クレジットカード)'<br> 'デフォルトは none: 指定なし です。' 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentMethod {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "domestic_bank_transfer")]
    DomesticBankTransfer,
    #[serde(rename = "abroad_bank_transfer")]
    AbroadBankTransfer,
    #[serde(rename = "account_transfer")]
    AccountTransfer,
    #[serde(rename = "credit_card")]
    CreditCard,
}

impl Default for PaymentMethod {
    fn default() -> PaymentMethod {
        Self::None
    }
}
/// '口座種別(ordinary: 普通、checking: 当座、earmarked: 納税準備預金、savings: 貯蓄、other: その他)'<br> '支払先指定時には無効'<br> 'デフォルトは ordinary: 普通 です' 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "ordinary")]
    Ordinary,
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "earmarked")]
    Earmarked,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "other")]
    Other,
}

impl Default for AccountType {
    fn default() -> AccountType {
        Self::Ordinary
    }
}

