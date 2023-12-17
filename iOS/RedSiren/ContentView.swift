import SharedTypes
import SwiftUI
import UIScreenExtension

struct SizePreferenceKey: PreferenceKey {
    static var defaultValue: CGSize = .zero
    static func reduce(value: inout CGSize, nextValue: () -> CGSize) {
        value = nextValue()
    }
}


struct ContentView: View {
    @ObservedObject var core: Core


    @Environment(\.safeAreaInsets) private var safeAreaInsets

    init(core: Core) {
        self.core = core
    }

    func viewSize(size: CGSize) {
        let dpi = UIScreen.pixelsPerInch ?? 1.0


        self.core.update(Event.createConfigAndConfigureApp(
            width: size.width,
            height: size.height,
            dpi: dpi,
            safe_areas: [
                self.safeAreaInsets.leading,
                self.safeAreaInsets.top,
                self.safeAreaInsets.trailing,
                self.safeAreaInsets.bottom
            ]
        ))
    }

    func introEv(ev: IntroEV) {
        self.core.update(Event.introEvent(ev))
    }

    func instrumentEv(ev: InstrumentEV) {
        self.core.update(Event.instrumentEvent(ev))
    }

    @ViewBuilder func ActivityView() -> some View {
        switch self.core.view.activity {
        case Activity.intro:
            IntroView(vm: self.core.view.intro, ev: self.introEv)
        case Activity.play:
            InstrumentView(vm: self.core.view.instrument, ev: self.instrumentEv)
        default:
            VStack {
                Text("Not implemented")
            }
        }
    }

    var body: some View {
        ZStack {
            AnyView(
                ActivityView()
            )
        }
            .ignoresSafeArea(.all)
            .statusBarHidden(true)
            .overlay(
                GeometryReader { proxy in
                    Color.clear.preference(key: SizePreferenceKey.self, value: proxy.frame(in: .global).size)
                }
                    .ignoresSafeArea(.all)
            )
            .onPreferenceChange(SizePreferenceKey.self) { value in
                self.viewSize(size: value)
            }
            .background(Color("Main"))
    }
}



struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(core: Core())
    }
}
