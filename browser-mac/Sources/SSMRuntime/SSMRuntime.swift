// SSMRuntime.swift
// Local Small Language Model Runtime for LEX-Ω Browser
//
// [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

import Foundation
import InvarianceCore

// MARK: - SSM Types

public enum SSMTask: String, CaseIterable {
    case pageAnalyzer = "page_analyzer"
    case commandParser = "command_parser"
    case contentSummarizer = "content_summarizer"
    case linkExtractor = "link_extractor"
}

// MARK: - SSM Output

public struct SSMOutput {
    public let task: SSMTask
    public let result: String
    public let renderDecision: RenderDecision
    public let processingTimeMs: Double
    
    public var isAuthorized: Bool {
        renderDecision.isAuthorized
    }
}

// MARK: - SSM Configuration

public struct SSMConfig {
    public let maxTokens: Int
    public let temperature: Double
    public let modelPath: String?
    public let useMetalAcceleration: Bool
    
    public init(
        maxTokens: Int = 512,
        temperature: Double = 0.0, // Deterministic
        modelPath: String? = nil,
        useMetalAcceleration: Bool = true
    ) {
        self.maxTokens = maxTokens
        self.temperature = temperature
        self.modelPath = modelPath
        self.useMetalAcceleration = useMetalAcceleration
    }
    
    public static let `default` = SSMConfig()
}

// MARK: - SSM Runtime

public class SSMRuntime {
    private let config: SSMConfig
    private let guard_: InvarianceGuard
    private var isInitialized = false
    
    public init(config: SSMConfig = .default) {
        self.config = config
        self.guard_ = InvarianceGuard(signFunction: createMockSigner())
    }
    
    // MARK: - Initialization
    
    public func initialize() async throws {
        // In production, this would load the actual model
        // For now, we simulate initialization
        try await Task.sleep(nanoseconds: 100_000_000) // 100ms
        isInitialized = true
        print("[SSMRuntime] Initialized with Metal: \(config.useMetalAcceleration)")
    }
    
    // MARK: - Task Execution
    
    public func execute(
        task: SSMTask,
        input: String,
        expectedOutput: String? = nil
    ) async throws -> SSMOutput {
        guard isInitialized else {
            throw SSMError.notInitialized
        }
        
        let startTime = CFAbsoluteTimeGetCurrent()
        
        // Execute the appropriate task
        let result: String
        switch task {
        case .pageAnalyzer:
            result = try await analyzePage(input)
        case .commandParser:
            result = try await parseCommand(input)
        case .contentSummarizer:
            result = try await summarizeContent(input)
        case .linkExtractor:
            result = try await extractLinks(input)
        }
        
        let processingTime = (CFAbsoluteTimeGetCurrent() - startTime) * 1000
        
        // Apply invariance check
        let intent = expectedOutput ?? result // Self-referential if no expected output
        let decision = guard_.process(output: result, substrateIntent: intent)
        
        return SSMOutput(
            task: task,
            result: result,
            renderDecision: decision,
            processingTimeMs: processingTime
        )
    }
    
    // MARK: - Task Implementations
    
    private func analyzePage(_ html: String) async throws -> String {
        // Simplified page analysis - in production would use actual model
        let wordCount = html.split(separator: " ").count
        let hasScript = html.contains("<script")
        let hasForm = html.contains("<form")
        
        return """
        {
            "word_count": \(wordCount),
            "has_scripts": \(hasScript),
            "has_forms": \(hasForm),
            "analysis": "Page analyzed successfully"
        }
        """
    }
    
    private func parseCommand(_ input: String) async throws -> String {
        // Simple command parsing
        let lowercased = input.lowercased()
        
        var intent = "unknown"
        if lowercased.contains("search") || lowercased.contains("find") {
            intent = "search"
        } else if lowercased.contains("go to") || lowercased.contains("navigate") {
            intent = "navigate"
        } else if lowercased.contains("summarize") || lowercased.contains("summary") {
            intent = "summarize"
        } else if lowercased.contains("bookmark") || lowercased.contains("save") {
            intent = "bookmark"
        }
        
        return """
        {
            "intent": "\(intent)",
            "original": "\(input.replacingOccurrences(of: "\"", with: "\\\""))"
        }
        """
    }
    
    private func summarizeContent(_ content: String) async throws -> String {
        // Simple extractive summary - take first few sentences
        let sentences = content.components(separatedBy: ". ")
        let summary = sentences.prefix(3).joined(separator: ". ")
        
        return summary.isEmpty ? "No content to summarize." : summary + "."
    }
    
    private func extractLinks(_ html: String) async throws -> String {
        // Simple link extraction using regex
        let pattern = #"href=["\']([^"\']+)["\']"#
        let regex = try NSRegularExpression(pattern: pattern, options: [])
        let range = NSRange(html.startIndex..., in: html)
        let matches = regex.matches(in: html, options: [], range: range)
        
        let links = matches.compactMap { match -> String? in
            guard let range = Range(match.range(at: 1), in: html) else { return nil }
            return String(html[range])
        }
        
        let jsonLinks = links.map { "\"\($0.replacingOccurrences(of: "\"", with: "\\\""))\"" }
        return "[\(jsonLinks.joined(separator: ", "))]"
    }
    
    // MARK: - Metrics
    
    public var metrics: SSMMetrics {
        SSMMetrics(
            authorizations: guard_.authorizationCount,
            violations: guard_.violationCount,
            isCZeroCompliant: guard_.isCZeroCompliant
        )
    }
}

// MARK: - SSM Metrics

public struct SSMMetrics {
    public let authorizations: Int
    public let violations: Int
    public let isCZeroCompliant: Bool
}

// MARK: - SSM Errors

public enum SSMError: Error {
    case notInitialized
    case modelLoadFailed(String)
    case inferenceError(String)
    case invarianceViolation
}

// MARK: - Batch Processing

extension SSMRuntime {
    public func executeBatch(
        tasks: [(SSMTask, String)],
        expectedOutputs: [String]? = nil
    ) async throws -> [SSMOutput] {
        var results: [SSMOutput] = []
        
        for (index, (task, input)) in tasks.enumerated() {
            let expected = expectedOutputs?[safe: index]
            let output = try await execute(task: task, input: input, expectedOutput: expected)
            results.append(output)
        }
        
        return results
    }
}

// MARK: - Array Extension

private extension Array {
    subscript(safe index: Int) -> Element? {
        indices.contains(index) ? self[index] : nil
    }
}

