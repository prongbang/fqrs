import Foundation

private let lib = dlopen("libfqrs.a", RTLD_LAZY)

private let decode_qr_code: @convention(c) (UnsafePointer<UInt8>?, Int32, Int32, Int32) -> UnsafeMutablePointer<Int8>? = dlsym(lib, "decode_qr_code").assumingMemoryBound(to: (@convention(c) (UnsafePointer<UInt8>?, Int32, Int32, Int32) -> UnsafeMutablePointer<Int8>?).self)

private let free_string: @convention(c) (UnsafeMutablePointer<Int8>?) -> Void = dlsym(lib, "free_string").assumingMemoryBound(to: (@convention(c) (UnsafeMutablePointer<Int8>?) -> Void).self)

public class QRDecoder {

     public init() {}

     func decodeQRCode(frameData: [UInt8], width: Int32, height: Int32) -> String? {
         let result = frameData.withUnsafeBufferPointer { ptr in
             decode_qr_code(ptr.baseAddress, width, height, Int32(frameData.count))
         }

         if let result = result {
             let string = String(cString: result)
             free_string(result)
             return string
         }

         return nil
     }
}