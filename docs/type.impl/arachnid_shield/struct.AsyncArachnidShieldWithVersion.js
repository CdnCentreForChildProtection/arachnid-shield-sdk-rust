(function() {var type_impls = {
"arachnid_shield":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsyncArachnidShieldWithVersion%3CV1%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#220-369\">source</a><a href=\"#impl-AsyncArachnidShieldWithVersion%3CV1%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html\" title=\"struct arachnid_shield::AsyncArachnidShieldWithVersion\">AsyncArachnidShieldWithVersion</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.V1.html\" title=\"struct arachnid_shield::V1\">V1</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.scan_media_from_bytes\" class=\"method\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#231-238\">source</a><h4 class=\"code-header\">pub async fn <a href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html#tymethod.scan_media_from_bytes\" class=\"fn\">scan_media_from_bytes</a>&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.79.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u8.html\">u8</a>&gt;&gt;&gt;(\n    &amp;self,\n    data: T,\n    mime_type: <a class=\"struct\" href=\"https://docs.rs/mime/0.3.17/mime/struct.Mime.html\" title=\"struct mime::Mime\">Mime</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.ScannedMedia.html\" title=\"struct arachnid_shield::ScannedMedia\">ScannedMedia</a>, <a class=\"enum\" href=\"arachnid_shield/enum.ArachnidShieldError.html\" title=\"enum arachnid_shield::ArachnidShieldError\">ArachnidShieldError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Given the contents of some media, and a mime type for it, scan the contents for CSAM using the Arachnid Shield API.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.scan_media_from_file\" class=\"method\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#241-258\">source</a><h4 class=\"code-header\">pub async fn <a href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html#tymethod.scan_media_from_file\" class=\"fn\">scan_media_from_file</a>&lt;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.79.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;&gt;(\n    &amp;self,\n    filepath: P\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.ScannedMedia.html\" title=\"struct arachnid_shield::ScannedMedia\">ScannedMedia</a>, <a class=\"enum\" href=\"arachnid_shield/enum.ArachnidShieldError.html\" title=\"enum arachnid_shield::ArachnidShieldError\">ArachnidShieldError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Given a path to a file, scan its contents for CSAM using the Arachnid Shield API.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.scan_media_from_bytes_with_config\" class=\"method\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#261-289\">source</a><h4 class=\"code-header\">pub async fn <a href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html#tymethod.scan_media_from_bytes_with_config\" class=\"fn\">scan_media_from_bytes_with_config</a>(\n    &amp;self,\n    params: <a class=\"struct\" href=\"arachnid_shield/struct.ScanMediaFromBytes.html\" title=\"struct arachnid_shield::ScanMediaFromBytes\">ScanMediaFromBytes</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.ScannedMedia.html\" title=\"struct arachnid_shield::ScannedMedia\">ScannedMedia</a>, <a class=\"enum\" href=\"arachnid_shield/enum.ArachnidShieldError.html\" title=\"enum arachnid_shield::ArachnidShieldError\">ArachnidShieldError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Given a <code>ScanMediaFromBytes</code> request, send it to the Arachnid Shield API.</p>\n</div></details><section id=\"method.download_and_scan_media_from_url\" class=\"method\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#291-297\">source</a><h4 class=\"code-header\">pub async fn <a href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html#tymethod.download_and_scan_media_from_url\" class=\"fn\">download_and_scan_media_from_url</a>(\n    &amp;self,\n    url: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.str.html\">str</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.ScannedMedia.html\" title=\"struct arachnid_shield::ScannedMedia\">ScannedMedia</a>, <a class=\"enum\" href=\"arachnid_shield/enum.ArachnidShieldError.html\" title=\"enum arachnid_shield::ArachnidShieldError\">ArachnidShieldError</a>&gt;</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.download_and_scan_media_from_url_with_config\" class=\"method\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#300-328\">source</a><h4 class=\"code-header\">pub async fn <a href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html#tymethod.download_and_scan_media_from_url_with_config\" class=\"fn\">download_and_scan_media_from_url_with_config</a>(\n    &amp;self,\n    params: <a class=\"struct\" href=\"arachnid_shield/struct.ScanMediaFromUrl.html\" title=\"struct arachnid_shield::ScanMediaFromUrl\">ScanMediaFromUrl</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.ScannedMedia.html\" title=\"struct arachnid_shield::ScannedMedia\">ScannedMedia</a>, <a class=\"enum\" href=\"arachnid_shield/enum.ArachnidShieldError.html\" title=\"enum arachnid_shield::ArachnidShieldError\">ArachnidShieldError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Given a <code>ScanMediaFromUrl</code> request, send it to the Arachnid Shield API.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.scan_pdq_hashes\" class=\"method\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#331-337\">source</a><h4 class=\"code-header\">pub async fn <a href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html#tymethod.scan_pdq_hashes\" class=\"fn\">scan_pdq_hashes</a>(\n    &amp;self,\n    data: &amp;[[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.79.0/std/primitive.array.html\">32</a>]]\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.ScannedPdqHashes.html\" title=\"struct arachnid_shield::ScannedPdqHashes\">ScannedPdqHashes</a>, <a class=\"enum\" href=\"arachnid_shield/enum.ArachnidShieldError.html\" title=\"enum arachnid_shield::ArachnidShieldError\">ArachnidShieldError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Given the PDQ of some media, scan the contents for CSAM using the Arachnid Shield API.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.scan_pdq_hashes_with_config\" class=\"method\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#340-368\">source</a><h4 class=\"code-header\">pub async fn <a href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html#tymethod.scan_pdq_hashes_with_config\" class=\"fn\">scan_pdq_hashes_with_config</a>(\n    &amp;self,\n    params: <a class=\"struct\" href=\"arachnid_shield/struct.ScanPdqHashes.html\" title=\"struct arachnid_shield::ScanPdqHashes\">ScanPdqHashes</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.79.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.ScannedPdqHashes.html\" title=\"struct arachnid_shield::ScannedPdqHashes\">ScannedPdqHashes</a>, <a class=\"enum\" href=\"arachnid_shield/enum.ArachnidShieldError.html\" title=\"enum arachnid_shield::ArachnidShieldError\">ArachnidShieldError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Given a ‘ScanPdqHashes’ request, send it to the Arachnid Shield API.</p>\n</div></details></div></details>",0,"arachnid_shield::client::AsyncArachnidShield"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsyncArachnidShieldWithVersion%3CV1%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/arachnid_shield/client.rs.html#52-76\">source</a><a href=\"#impl-AsyncArachnidShieldWithVersion%3CV1%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html\" title=\"struct arachnid_shield::AsyncArachnidShieldWithVersion\">AsyncArachnidShieldWithVersion</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.V1.html\" title=\"struct arachnid_shield::V1\">V1</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/arachnid_shield/client.rs.html#54-75\">source</a><h4 class=\"code-header\">pub fn <a href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html#tymethod.new\" class=\"fn\">new</a>(user: <a class=\"struct\" href=\"arachnid_shield/struct.ApiUser.html\" title=\"struct arachnid_shield::ApiUser\">ApiUser</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Create a new Async ArachnidShield consumer given the API user.</p>\n</div></details></div></details>",0,"arachnid_shield::client::AsyncArachnidShield"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-AsyncArachnidShieldWithVersion%3CV1%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#22-29\">source</a><a href=\"#impl-Debug-for-AsyncArachnidShieldWithVersion%3CV1%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.79.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"arachnid_shield/struct.AsyncArachnidShieldWithVersion.html\" title=\"struct arachnid_shield::AsyncArachnidShieldWithVersion\">AsyncArachnidShieldWithVersion</a>&lt;<a class=\"struct\" href=\"arachnid_shield/struct.V1.html\" title=\"struct arachnid_shield::V1\">V1</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/arachnid_shield/api/v1.rs.html#23-28\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.79.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.79.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.79.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.79.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","arachnid_shield::client::AsyncArachnidShield"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()