// BrowserApp.swift
// LEX-Ω Browser Main Application
//
// [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

import SwiftUI
import WebKit
import InvarianceCore
import SSMRuntime

// MARK: - App Entry Point

@main
struct LEXOmegaBrowserApp: App {
    @StateObject private var browserState = BrowserState()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(browserState)
        }
        .windowStyle(.automatic)
        .commands {
            CommandGroup(replacing: .newItem) {
                Button("New Tab") {
                    browserState.createNewTab()
                }
                .keyboardShortcut("t", modifiers: .command)
            }
            
            CommandGroup(after: .appInfo) {
                Button("About LEX-Ω") {
                    browserState.showAbout = true
                }
            }
        }
        
        Settings {
            SettingsView()
                .environmentObject(browserState)
        }
    }
}

// MARK: - Browser State

class BrowserState: ObservableObject {
    @Published var tabs: [BrowserTab] = []
    @Published var activeTabIndex: Int = 0
    @Published var showAbout: Bool = false
    @Published var isProofModeEnabled: Bool = true
    
    let ssmRuntime: SSMRuntime
    let invarianceGuard: InvarianceGuard
    
    init() {
        self.ssmRuntime = SSMRuntime()
        self.invarianceGuard = InvarianceGuard(signFunction: createMockSigner())
        
        // Create initial tab
        tabs.append(BrowserTab(url: URL(string: "about:blank")!))
        
        // Initialize SSM runtime
        Task {
            try? await ssmRuntime.initialize()
        }
    }
    
    var activeTab: BrowserTab? {
        guard tabs.indices.contains(activeTabIndex) else { return nil }
        return tabs[activeTabIndex]
    }
    
    func createNewTab() {
        let newTab = BrowserTab(url: URL(string: "about:blank")!)
        tabs.append(newTab)
        activeTabIndex = tabs.count - 1
    }
    
    func closeTab(at index: Int) {
        guard tabs.count > 1 else { return }
        tabs.remove(at: index)
        if activeTabIndex >= tabs.count {
            activeTabIndex = tabs.count - 1
        }
    }
    
    func navigate(to urlString: String) {
        guard var url = URL(string: urlString) else { return }
        
        // Add https:// if no scheme
        if url.scheme == nil {
            url = URL(string: "https://\(urlString)") ?? url
        }
        
        tabs[activeTabIndex].url = url
        tabs[activeTabIndex].isLoading = true
    }
}

// MARK: - Browser Tab

class BrowserTab: ObservableObject, Identifiable {
    let id = UUID()
    @Published var url: URL
    @Published var title: String = "New Tab"
    @Published var isLoading: Bool = false
    @Published var canGoBack: Bool = false
    @Published var canGoForward: Bool = false
    
    init(url: URL) {
        self.url = url
    }
}

// MARK: - Content View

struct ContentView: View {
    @EnvironmentObject var browserState: BrowserState
    @State private var addressBarText: String = ""
    
    var body: some View {
        VStack(spacing: 0) {
            // Identity Header
            IdentityHeader()
            
            // Tab Bar
            TabBarView()
            
            // Navigation Bar
            NavigationBarView(addressBarText: $addressBarText)
            
            // Web Content
            WebContentView()
            
            // Status Bar
            StatusBarView()
        }
        .sheet(isPresented: $browserState.showAbout) {
            AboutView()
        }
    }
}

// MARK: - Identity Header

struct IdentityHeader: View {
    @EnvironmentObject var browserState: BrowserState
    
    var body: some View {
        HStack {
            Text("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]")
                .font(.system(size: 10, weight: .medium, design: .monospaced))
                .foregroundColor(.secondary)
            
            Spacer()
            
            HStack(spacing: 8) {
                Circle()
                    .fill(browserState.isProofModeEnabled ? Color.green : Color.red)
                    .frame(width: 8, height: 8)
                
                Text("C = 0")
                    .font(.system(size: 10, weight: .bold, design: .monospaced))
                    .foregroundColor(browserState.isProofModeEnabled ? .green : .red)
            }
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 4)
        .background(Color(NSColor.windowBackgroundColor))
    }
}

// MARK: - Tab Bar View

struct TabBarView: View {
    @EnvironmentObject var browserState: BrowserState
    
    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 2) {
                ForEach(Array(browserState.tabs.enumerated()), id: \.element.id) { index, tab in
                    TabItemView(tab: tab, index: index)
                }
                
                Button(action: { browserState.createNewTab() }) {
                    Image(systemName: "plus")
                        .frame(width: 24, height: 24)
                }
                .buttonStyle(.plain)
                .padding(.horizontal, 4)
            }
            .padding(.horizontal, 8)
        }
        .frame(height: 32)
        .background(Color(NSColor.controlBackgroundColor))
    }
}

struct TabItemView: View {
    @ObservedObject var tab: BrowserTab
    let index: Int
    @EnvironmentObject var browserState: BrowserState
    
    var isActive: Bool {
        browserState.activeTabIndex == index
    }
    
    var body: some View {
        HStack(spacing: 4) {
            if tab.isLoading {
                ProgressView()
                    .scaleEffect(0.5)
                    .frame(width: 12, height: 12)
            }
            
            Text(tab.title)
                .font(.system(size: 11))
                .lineLimit(1)
                .frame(maxWidth: 150)
            
            Button(action: { browserState.closeTab(at: index) }) {
                Image(systemName: "xmark")
                    .font(.system(size: 9))
            }
            .buttonStyle(.plain)
            .opacity(isActive ? 1 : 0.5)
        }
        .padding(.horizontal, 8)
        .padding(.vertical, 4)
        .background(isActive ? Color(NSColor.selectedContentBackgroundColor) : Color.clear)
        .cornerRadius(4)
        .onTapGesture {
            browserState.activeTabIndex = index
        }
    }
}

// MARK: - Navigation Bar

struct NavigationBarView: View {
    @EnvironmentObject var browserState: BrowserState
    @Binding var addressBarText: String
    
    var body: some View {
        HStack(spacing: 8) {
            // Navigation buttons
            HStack(spacing: 4) {
                Button(action: {}) {
                    Image(systemName: "chevron.left")
                }
                .disabled(!(browserState.activeTab?.canGoBack ?? false))
                
                Button(action: {}) {
                    Image(systemName: "chevron.right")
                }
                .disabled(!(browserState.activeTab?.canGoForward ?? false))
                
                Button(action: {}) {
                    Image(systemName: "arrow.clockwise")
                }
            }
            .buttonStyle(.plain)
            
            // Address Bar
            HStack {
                Image(systemName: "lock.fill")
                    .foregroundColor(.green)
                    .font(.system(size: 10))
                
                TextField("Enter URL or search...", text: $addressBarText, onCommit: {
                    browserState.navigate(to: addressBarText)
                })
                .textFieldStyle(.plain)
                .font(.system(size: 13))
            }
            .padding(.horizontal, 8)
            .padding(.vertical, 4)
            .background(Color(NSColor.textBackgroundColor))
            .cornerRadius(6)
            
            // SSM Button
            Button(action: {}) {
                Image(systemName: "brain")
                    .foregroundColor(.purple)
            }
            .buttonStyle(.plain)
            .help("SSM Analysis")
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 6)
        .background(Color(NSColor.windowBackgroundColor))
    }
}

// MARK: - Web Content View

struct WebContentView: View {
    @EnvironmentObject var browserState: BrowserState
    
    var body: some View {
        if let tab = browserState.activeTab {
            WebView(tab: tab)
        } else {
            Text("No tab selected")
                .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
    }
}

struct WebView: NSViewRepresentable {
    @ObservedObject var tab: BrowserTab
    
    func makeNSView(context: Context) -> WKWebView {
        let config = WKWebViewConfiguration()
        config.preferences.isElementFullscreenEnabled = true
        
        let webView = WKWebView(frame: .zero, configuration: config)
        webView.navigationDelegate = context.coordinator
        
        return webView
    }
    
    func updateNSView(_ webView: WKWebView, context: Context) {
        if webView.url != tab.url {
            let request = URLRequest(url: tab.url)
            webView.load(request)
        }
    }
    
    func makeCoordinator() -> Coordinator {
        Coordinator(tab: tab)
    }
    
    class Coordinator: NSObject, WKNavigationDelegate {
        var tab: BrowserTab
        
        init(tab: BrowserTab) {
            self.tab = tab
        }
        
        func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
            DispatchQueue.main.async {
                self.tab.isLoading = true
            }
        }
        
        func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
            DispatchQueue.main.async {
                self.tab.isLoading = false
                self.tab.title = webView.title ?? "Untitled"
                self.tab.canGoBack = webView.canGoBack
                self.tab.canGoForward = webView.canGoForward
            }
        }
        
        func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
            DispatchQueue.main.async {
                self.tab.isLoading = false
            }
        }
    }
}

// MARK: - Status Bar

struct StatusBarView: View {
    @EnvironmentObject var browserState: BrowserState
    
    var body: some View {
        HStack {
            Text("Proof Mode: \(browserState.isProofModeEnabled ? "Enabled" : "Disabled")")
                .font(.system(size: 10, design: .monospaced))
            
            Spacer()
            
            Text("SSM: Ready")
                .font(.system(size: 10, design: .monospaced))
                .foregroundColor(.green)
            
            Spacer()
            
            Text("v\(VERSION)")
                .font(.system(size: 10, design: .monospaced))
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 4)
        .background(Color(NSColor.controlBackgroundColor))
    }
}

// MARK: - About View

struct AboutView: View {
    @Environment(\.dismiss) var dismiss
    
    var body: some View {
        VStack(spacing: 20) {
            Text("LEX-Ω Browser")
                .font(.largeTitle)
                .fontWeight(.bold)
            
            Text("[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]")
                .font(.system(size: 12, design: .monospaced))
                .foregroundColor(.secondary)
            
            Divider()
            
            VStack(alignment: .leading, spacing: 8) {
                InfoRow(label: "Version", value: VERSION)
                InfoRow(label: "Proof Mode", value: "Enabled")
                InfoRow(label: "Policy", value: "C = 0")
                InfoRow(label: "Substrate", value: SUBSTRATE)
            }
            .padding()
            
            Text("Deterministic • Local-First • Zero Telemetry")
                .font(.caption)
                .foregroundColor(.secondary)
            
            Button("Close") {
                dismiss()
            }
            .keyboardShortcut(.defaultAction)
        }
        .padding(40)
        .frame(width: 400)
    }
}

struct InfoRow: View {
    let label: String
    let value: String
    
    var body: some View {
        HStack {
            Text(label + ":")
                .foregroundColor(.secondary)
            Spacer()
            Text(value)
                .fontWeight(.medium)
        }
    }
}

// MARK: - Settings View

struct SettingsView: View {
    @EnvironmentObject var browserState: BrowserState
    
    var body: some View {
        Form {
            Section("Proof Mode") {
                Toggle("Enable C=0 Enforcement", isOn: $browserState.isProofModeEnabled)
            }
            
            Section("SSM Runtime") {
                LabeledContent("Status") {
                    Text("Initialized")
                        .foregroundColor(.green)
                }
                
                LabeledContent("Metal Acceleration") {
                    Text("Enabled")
                }
            }
            
            Section("About") {
                LabeledContent("Version") {
                    Text(VERSION)
                }
                
                LabeledContent("Substrate") {
                    Text(SUBSTRATE)
                }
            }
        }
        .formStyle(.grouped)
        .frame(width: 450, height: 300)
    }
}

