import SharedTypes
import SwiftUI


struct ContentView: View {
    @ObservedObject var core: Core

    init(core: Core) {
        self.core = core
    }

    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello siren")
        }
    }
}


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(core: Core())
    }
}
