# SweetCandy 🍬

Sweet Candy は、脆弱なサービスや魅力的な環境をシミュレートすることにより、攻撃者の行動を引き付け、分析するように設計されたハニーポットソフトウェアです。このタイプのソリューションは、サイバーセキュリティ プロジェクトで、攻撃方法に関する情報を収集したり、エクスプロイト ベクトルを特定したり、これらの相互作用から学んだ教訓を利用して防御を強化したりするために頻繁に使用されます。

<details>
<summary><b>EN 🇬🇧</b></summary>
<br>
Sweet Candy is a honeypot software designed to attract and analyze the behavior of attackers by simulating vulnerable services or attractive environments. This type of solution is frequently used in cybersecurity projects to collect information on attack methods, identify exploit vectors, or strengthen defenses by drawing on the lessons learned from these interactions.
</details>

![](./.github/banner.png)

> [!IMPORTANT]  
> 管理パネルは現在作成中です
> The admin panel is currently under construction

# サポートされている脆弱性
(Supported Vulnerabilities)

SweetCandy は、Web の脆弱性の特定と分析に特化したソフトウェアです。危険な環境をシミュレートするように設計されており、SQL インジェクション、XSS の脆弱性、構成エラーなど、Web アプリケーションの潜在的な脆弱性を検出できます。このタイプのソフトウェアは、実際の脅威に対するシステムのセキュリティを評価および強化するための強力なツールです。

ただし、安全で最適な使用を確実にするためには、SweetCandy をプロキシの後ろにデプロイすることが不可欠です。これにより、ツールへのアクセスが保護されるだけでなく、悪意のある接続が除外され、インタラクションが匿名化され、通過するデータの厳密な制御が維持されます。したがって、適切に構成されたプロキシは、攻撃者へのツールの直接暴露を回避しながら、追加の保護層を追加する上で重要な役割を果たします。

SweetCandy を制御された環境に置くことで、インフラストラクチャのリスクを制限しながら、分析のセキュリティを最大化します。


<details>
<summary><b>EN 🇬🇧</b></summary>
<br>
SweetCandy is a software specialized in the identification and analysis of web vulnerabilities. Designed to simulate risky environments, it can detect potential vulnerabilities in web applications, such as SQL injections, XSS vulnerabilities, or configuration errors. This type of software is a powerful tool for assessing and strengthening the security of systems against real threats.
<br><br>
However, to ensure safe and optimal use, it is essential to deploy SweetCandy behind a proxy. This not only protects access to the tool, but also filters out malicious connections, anonymizes interactions, and maintains strict control over data that passes through. A well-configured proxy therefore plays a crucial role in adding an additional layer of protection, while avoiding any direct exposure of the tool to attackers.
<br><br>
By placing SweetCandy in a controlled environment, you maximize the security of your analytics while limiting risks to your infrastructure.
</details>


## 一般的なスキャン
(Common Scans)

| 脆弱性 (Vulnerability) | パス (Path) | 悪用例 (Exploit) |
| --- | --- | --- |
| Git file exposed exploit | `/vulns/.git*` | [liamg/gitjacker](https://github.com/liamg/gitjacker)


## ワードプレス
(WordPress)

| 脆弱性 (Vulnerability) | パス (Path) | 悪用例 (Exploit) |
| --- | --- | --- |
| Fake Wp login page | `/vulns/wp-login.php`, `/vulns/wp-admin` | ... |


# 向け
(Contributing)

私たちの仕事に貢献したい場合は、チケットまたはプルリクエストを開き、パネルに新しい脆弱性に対する新しいサポートを追加するように依頼してください

<details>
<summary><b>EN 🇬🇧</b></summary>
<br>
If you want to contribute to our work, please just open an issue or a pull request and ask to add a new support for a new vulnerability in the panel
</details>
