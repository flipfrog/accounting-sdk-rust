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
pub struct ExpenseApplicationResponseExpenseApplication {
    /// 経費申請ID
    #[serde(rename = "id")]
    pub id: i32,
    /// 事業所ID
    #[serde(rename = "company_id")]
    pub company_id: i32,
    /// 申請タイトル
    #[serde(rename = "title")]
    pub title: String,
    /// 申請日 (yyyy-mm-dd)
    #[serde(rename = "issue_date")]
    pub issue_date: String,
    /// 備考
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// 合計金額
    #[serde(rename = "total_amount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i32>,
    /// 申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し)
    #[serde(rename = "status")]
    pub status: Status,
    /// 部門ID
    #[serde(rename = "section_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub section_id: Option<Option<i32>>,
    /// メモタグID
    #[serde(rename = "tag_ids", skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<i32>>,
    /// 経費申請の項目行一覧（配列）
    #[serde(rename = "expense_application_lines")]
    pub expense_application_lines: Vec<crate::models::ExpenseApplicationResponseExpenseApplicationExpenseApplicationLinesInner>,
    /// 取引ID (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_idが表示されます)
    #[serde(rename = "deal_id", deserialize_with = "Option::deserialize")]
    pub deal_id: Option<i32>,
    /// 取引ステータス (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_statusが表示されます settled:精算済み, unsettled:清算待ち)
    #[serde(rename = "deal_status", deserialize_with = "Option::deserialize")]
    pub deal_status: Option<DealStatus>,
    /// 申請者のユーザーID
    #[serde(rename = "applicant_id")]
    pub applicant_id: i32,
    /// 承認者（配列）   承認ステップのresource_typeがunspecified (指定なし)の場合はapproversはレスポンスに含まれません。   しかし、resource_typeがunspecifiedの承認ステップにおいて誰かが承認・却下・差し戻しのいずれかのアクションを取った後は、   approversはレスポンスに含まれるようになります。   その場合approversにはアクションを行ったステップのIDとアクションを行ったユーザーのIDが含まれます。
    #[serde(rename = "approvers")]
    pub approvers: Vec<crate::models::ExpenseApplicationResponseExpenseApplicationApproversInner>,
    /// 申請No.
    #[serde(rename = "application_number")]
    pub application_number: String,
    /// 申請経路ID
    #[serde(rename = "approval_flow_route_id")]
    pub approval_flow_route_id: i32,
    /// 経費申請のコメント一覧（配列）
    #[serde(rename = "comments")]
    pub comments: Vec<crate::models::ExpenseApplicationResponseExpenseApplicationCommentsInner>,
    /// 経費申請の承認履歴（配列）
    #[serde(rename = "approval_flow_logs")]
    pub approval_flow_logs: Vec<crate::models::ExpenseApplicationResponseExpenseApplicationApprovalFlowLogsInner>,
    /// 現在承認ステップID
    #[serde(rename = "current_step_id", deserialize_with = "Option::deserialize")]
    pub current_step_id: Option<i32>,
    /// 現在のround。差し戻し等により申請がstepの最初からやり直しになるとroundの値が増えます。
    #[serde(rename = "current_round")]
    pub current_round: i32,
    /// セグメント１ID。セグメント１が使用可能なプランの時のみレスポンスに含まれます。
    #[serde(rename = "segment_1_tag_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub segment_1_tag_id: Option<Option<i64>>,
    /// セグメント２ID。セグメント２が使用可能なプランの時のみレスポンスに含まれます。
    #[serde(rename = "segment_2_tag_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub segment_2_tag_id: Option<Option<i64>>,
    /// セグメント３ID。セグメント３が使用可能なプランの時のみレスポンスに含まれます。
    #[serde(rename = "segment_3_tag_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub segment_3_tag_id: Option<Option<i64>>,
}

impl ExpenseApplicationResponseExpenseApplication {
    pub fn new(id: i32, company_id: i32, title: String, issue_date: String, status: Status, expense_application_lines: Vec<crate::models::ExpenseApplicationResponseExpenseApplicationExpenseApplicationLinesInner>, deal_id: Option<i32>, deal_status: Option<DealStatus>, applicant_id: i32, approvers: Vec<crate::models::ExpenseApplicationResponseExpenseApplicationApproversInner>, application_number: String, approval_flow_route_id: i32, comments: Vec<crate::models::ExpenseApplicationResponseExpenseApplicationCommentsInner>, approval_flow_logs: Vec<crate::models::ExpenseApplicationResponseExpenseApplicationApprovalFlowLogsInner>, current_step_id: Option<i32>, current_round: i32) -> ExpenseApplicationResponseExpenseApplication {
        ExpenseApplicationResponseExpenseApplication {
            id,
            company_id,
            title,
            issue_date,
            description: None,
            total_amount: None,
            status,
            section_id: None,
            tag_ids: None,
            expense_application_lines,
            deal_id,
            deal_status,
            applicant_id,
            approvers,
            application_number,
            approval_flow_route_id,
            comments,
            approval_flow_logs,
            current_step_id,
            current_round,
            segment_1_tag_id: None,
            segment_2_tag_id: None,
            segment_3_tag_id: None,
        }
    }
}

/// 申請ステータス(draft:下書き, in_progress:申請中, approved:承認済, rejected:却下, feedback:差戻し)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "feedback")]
    Feedback,
}

impl Default for Status {
    fn default() -> Status {
        Self::Draft
    }
}
/// 取引ステータス (申請ステータス:statusがapprovedで、取引が存在する時のみdeal_statusが表示されます settled:精算済み, unsettled:清算待ち)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DealStatus {
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "unsettled")]
    Unsettled,
}

impl Default for DealStatus {
    fn default() -> DealStatus {
        Self::Settled
    }
}

