import Foundation

// 変数定義
var variable: String = "Hello again!"
variable = variable + "!"
print(variable)

// 定数
let constant: String = "Hello again!"
// constant = constant + "!"　コンパイルエラー
print(constant)

// 型推論
var implicit: String = "Hello again!"
implicit = implicit + "!"
print(implicit)

var implicit2  = 1
print(type(of: implicit2))

// 型変換
var lable = "横幅:"
var width: Float = 44.0
var widthLabel: String = lable + String(width)
print(widthLabel)

// 埋め込み
var appleCount = 3
print("私は\(appleCount)個のリンゴを食べました。")
