import Foundation
import UIKit
import SharedTypes

@MainActor
class Core: ObservableObject {
    @Published var view: ViewModel
    
    init() {
        self.view = try! .bincodeDeserialize(input: [UInt8](RedSiren.view()))
    }
    
    func update(_ event: Event) {
        let effects = [UInt8](processEvent(Data(try! event.bincodeSerialize())))
        
        let requests: [Request] = try! .bincodeDeserialize(input: effects)
        for request in requests {
            processEffect(request)
        }
    }
    
    func processEffect(_ request: Request) {
        switch request.effect {
        case .render:
            view = try! .bincodeDeserialize(input: [UInt8](RedSiren.view()))
        case .keyValue(.read):
            let response = KeyValueOutput.read(.none)
            
            let effects = [UInt8](handleResponse(Data(request.uuid), Data(try! response.bincodeSerialize())))
            
            let requests: [Request] = try! .bincodeDeserialize(input: effects)
            for request in requests {
                processEffect(request)
            }
        case .keyValue(.write):
            let response = KeyValueOutput.write(false)
            
            let effects = [UInt8](handleResponse(Data(request.uuid), Data(try! response.bincodeSerialize())))
            
            let requests: [Request] = try! .bincodeDeserialize(input: effects)
            for request in requests {
                processEffect(request)
            }
        }
    }
}

func get_platform() -> String {
    return UIDevice.current.systemName + " " + UIDevice.current.systemVersion
}
